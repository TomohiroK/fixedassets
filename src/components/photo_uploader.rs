use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;
use crate::i18n::use_i18n;
use crate::models::photo::AssetPhoto;
use crate::stores::photo_store;

/// Max image dimension (longest side) after compression
const MAX_DIMENSION: u32 = 1200;
/// Thumbnail max dimension
const THUMB_DIMENSION: u32 = 200;
/// JPEG quality (0.0 - 1.0)
const JPEG_QUALITY: f64 = 0.75;
const THUMB_QUALITY: f64 = 0.6;

/// Compress an image file using Canvas, returns (full_data_url, thumb_data_url, size_bytes)
async fn compress_image(file: web_sys::File) -> Result<(String, String, u32), String> {
    let array_buffer = read_file_as_data_url(&file).await?;

    let img = load_image(&array_buffer).await?;

    let (orig_w, orig_h) = (img.natural_width(), img.natural_height());

    // Full size
    let (fw, fh) = fit_dimensions(orig_w, orig_h, MAX_DIMENSION);
    let full_url = draw_to_canvas(&img, fw, fh, JPEG_QUALITY)?;
    let size = full_url.len() as u32;

    // Thumbnail
    let (tw, th) = fit_dimensions(orig_w, orig_h, THUMB_DIMENSION);
    let thumb_url = draw_to_canvas(&img, tw, th, THUMB_QUALITY)?;

    Ok((full_url, thumb_url, size))
}

fn fit_dimensions(w: u32, h: u32, max_dim: u32) -> (u32, u32) {
    if w <= max_dim && h <= max_dim {
        return (w, h);
    }
    if w >= h {
        let new_w = max_dim;
        let new_h = (h as f64 * max_dim as f64 / w as f64) as u32;
        (new_w, new_h.max(1))
    } else {
        let new_h = max_dim;
        let new_w = (w as f64 * max_dim as f64 / h as f64) as u32;
        (new_w.max(1), new_h)
    }
}

async fn read_file_as_data_url(file: &web_sys::File) -> Result<String, String> {
    let (tx, rx) = futures_channel::oneshot::channel::<Result<String, String>>();
    let tx = Rc::new(RefCell::new(Some(tx)));

    let reader = web_sys::FileReader::new().map_err(|_| "FileReader creation failed")?;

    let tx_s = Rc::clone(&tx);
    let on_load = Closure::wrap(Box::new(move |event: web_sys::Event| {
        let target = event.target().unwrap();
        let reader: web_sys::FileReader = target.unchecked_into();
        let result = reader.result().unwrap();
        let data_url: String = result.as_string().unwrap_or_default();
        if let Some(sender) = tx_s.borrow_mut().take() {
            let _ = sender.send(Ok(data_url));
        }
    }) as Box<dyn FnMut(_)>);

    let tx_e = Rc::clone(&tx);
    let on_error = Closure::wrap(Box::new(move |_event: web_sys::Event| {
        if let Some(sender) = tx_e.borrow_mut().take() {
            let _ = sender.send(Err("File read failed".to_string()));
        }
    }) as Box<dyn FnMut(_)>);

    reader.set_onload(Some(on_load.as_ref().unchecked_ref()));
    reader.set_onerror(Some(on_error.as_ref().unchecked_ref()));

    reader.read_as_data_url(file).map_err(|_| "read_as_data_url failed")?;

    on_load.forget();
    on_error.forget();

    rx.await.map_err(|_| "Channel error".to_string())?
}

async fn load_image(data_url: &str) -> Result<web_sys::HtmlImageElement, String> {
    let (tx, rx) = futures_channel::oneshot::channel::<Result<web_sys::HtmlImageElement, String>>();
    let tx = Rc::new(RefCell::new(Some(tx)));

    let img = web_sys::HtmlImageElement::new().map_err(|_| "Image creation failed")?;

    let tx_s = Rc::clone(&tx);
    let img_clone = img.clone();
    let on_load = Closure::wrap(Box::new(move |_event: web_sys::Event| {
        if let Some(sender) = tx_s.borrow_mut().take() {
            let _ = sender.send(Ok(img_clone.clone()));
        }
    }) as Box<dyn FnMut(_)>);

    let tx_e = Rc::clone(&tx);
    let on_error = Closure::wrap(Box::new(move |_event: web_sys::Event| {
        if let Some(sender) = tx_e.borrow_mut().take() {
            let _ = sender.send(Err("Image load failed".to_string()));
        }
    }) as Box<dyn FnMut(_)>);

    img.set_onload(Some(on_load.as_ref().unchecked_ref()));
    img.set_onerror(Some(on_error.as_ref().unchecked_ref()));
    img.set_src(data_url);

    on_load.forget();
    on_error.forget();

    rx.await.map_err(|_| "Channel error".to_string())?
}

fn draw_to_canvas(img: &web_sys::HtmlImageElement, w: u32, h: u32, quality: f64) -> Result<String, String> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas: web_sys::HtmlCanvasElement = document
        .create_element("canvas")
        .map_err(|_| "Canvas creation failed")?
        .unchecked_into();
    canvas.set_width(w);
    canvas.set_height(h);

    let ctx: web_sys::CanvasRenderingContext2d = canvas
        .get_context("2d")
        .map_err(|_| "2d context failed")?
        .ok_or("No 2d context")?
        .unchecked_into();

    ctx.draw_image_with_html_image_element_and_dw_and_dh(img, 0.0, 0.0, w as f64, h as f64)
        .map_err(|_| "drawImage failed")?;

    canvas
        .to_data_url_with_type_and_encoder_options("image/jpeg", &JsValue::from_f64(quality))
        .map_err(|_| "toDataURL failed".to_string())
}

#[component]
pub fn PhotoUploader(
    asset_id: String,
    #[prop(into)] on_photo_added: Callback<AssetPhoto>,
) -> impl IntoView {
    let i18n = use_i18n();
    let is_uploading = RwSignal::new(false);
    let upload_count = RwSignal::new(0u32);
    let upload_total = RwSignal::new(0u32);
    let error_msg = RwSignal::new(Option::<String>::None);

    let asset_id_for_input = asset_id.clone();
    let on_photo_added_for_input = on_photo_added.clone();

    let handle_files = move |files: web_sys::FileList| {
        let total = files.length();
        if total == 0 {
            return;
        }
        upload_total.set(total);
        upload_count.set(0);
        is_uploading.set(true);
        error_msg.set(None);

        for idx in 0..total {
            if let Some(file) = files.get(idx) {
                let asset_id = asset_id_for_input.clone();
                let on_added = on_photo_added_for_input.clone();
                let filename = file.name();

                leptos::task::spawn_local(async move {
                    match compress_image(file).await {
                        Ok((full_url, thumb_url, size)) => {
                            let photo = AssetPhoto::new(
                                asset_id,
                                full_url,
                                thumb_url,
                                filename,
                                size,
                            );
                            match photo_store::save_photo(&photo).await {
                                Ok(()) => {
                                    on_added.run(photo);
                                }
                                Err(e) => {
                                    log::error!("Save photo error: {}", e);
                                    error_msg.set(Some(e));
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Compress error: {}", e);
                            error_msg.set(Some(e));
                        }
                    }
                    upload_count.update(|c| *c += 1);
                    if upload_count.get() >= upload_total.get() {
                        is_uploading.set(false);
                    }
                });
            }
        }
    };

    let handle_files_clone = handle_files.clone();

    view! {
        <div class="space-y-2">
            // Upload buttons
            <div class="flex gap-2">
                // File picker (PC + Mobile gallery)
                <label class="flex-1 flex items-center justify-center gap-2 py-3 border-2 border-dashed border-gray-300 rounded-lg text-sm text-gray-500 active:bg-gray-50 cursor-pointer">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                    </svg>
                    {move || i18n.t("photo.select")}
                    <input
                        type="file"
                        accept="image/*"
                        multiple=true
                        class="hidden"
                        on:change={
                            let hf = handle_files.clone();
                            move |ev: web_sys::Event| {
                                let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                                if let Some(files) = target.files() {
                                    hf(files);
                                }
                                target.set_value("");
                            }
                        }
                    />
                </label>
                // Camera capture (mobile only)
                <label class="flex items-center justify-center gap-1 px-4 py-3 border-2 border-dashed border-gray-300 rounded-lg text-sm text-gray-500 active:bg-gray-50 cursor-pointer">
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z"/>
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 13a3 3 0 11-6 0 3 3 0 016 0z"/>
                    </svg>
                    <input
                        type="file"
                        accept="image/*"
                        capture="environment"
                        class="hidden"
                        on:change={
                            let hf = handle_files_clone.clone();
                            move |ev: web_sys::Event| {
                                let target: web_sys::HtmlInputElement = ev.target().unwrap().unchecked_into();
                                if let Some(files) = target.files() {
                                    hf(files);
                                }
                                target.set_value("");
                            }
                        }
                    />
                </label>
            </div>

            // Upload progress
            {move || if is_uploading.get() {
                Some(view! {
                    <div class="flex items-center gap-2 text-sm text-blue-600">
                        <svg class="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                        </svg>
                        {move || format!("{}/{}", upload_count.get(), upload_total.get())}
                        " " {move || i18n.t("photo.uploading")}
                    </div>
                })
            } else {
                None
            }}

            // Error
            {move || error_msg.get().map(|msg| view! {
                <p class="text-xs text-red-500">{msg}</p>
            })}
        </div>
    }
}

#[component]
pub fn PhotoGallery(
    asset_id: String,
    #[prop(optional)] editable: bool,
) -> impl IntoView {
    let i18n = use_i18n();
    let refresh = RwSignal::new(0u32);
    let selected_photo = RwSignal::new(Option::<AssetPhoto>::None);

    let asset_id_res = asset_id.clone();
    let photos = LocalResource::new(move || {
        let aid = asset_id_res.clone();
        refresh.get();
        async move {
            photo_store::get_photos_for_asset(&aid).await.unwrap_or_default()
        }
    });

    let asset_id_up = asset_id.clone();
    let on_photo_added = Callback::new(move |_photo: AssetPhoto| {
        refresh.update(|v| *v += 1);
    });

    view! {
        <div class="space-y-3">
            // Uploader (only in edit mode)
            {if editable {
                let aid = asset_id_up.clone();
                Some(view! {
                    <PhotoUploader asset_id=aid on_photo_added=on_photo_added />
                })
            } else {
                None
            }}

            // Thumbnail grid
            <Suspense fallback=move || view! { <div></div> }>
                {move || {
                    photos.get().map(|data| {
                        let photos_vec: Vec<AssetPhoto> = (*data).clone();
                        if photos_vec.is_empty() {
                            if !editable {
                                return view! { <div></div> }.into_any();
                            }
                            return view! {
                                <p class="text-xs text-gray-400 text-center py-2">{move || i18n.t("photo.no_photos")}</p>
                            }.into_any();
                        }

                        let count = photos_vec.len();
                        view! {
                            <div>
                                <p class="text-xs text-gray-400 mb-1">{count} " " {move || i18n.t("photo.photos")}</p>
                                <div class="grid grid-cols-4 gap-1.5">
                                    {photos_vec.into_iter().map(|photo| {
                                        let thumb = photo.thumbnail_url.clone();
                                        let photo_for_click = photo.clone();
                                        view! {
                                            <button
                                                type="button"
                                                class="aspect-square rounded-lg overflow-hidden bg-gray-100 active:opacity-75"
                                                on:click=move |_| selected_photo.set(Some(photo_for_click.clone()))
                                            >
                                                <img src=thumb.clone() class="w-full h-full object-cover" />
                                            </button>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        }.into_any()
                    })
                }}
            </Suspense>

            // Fullscreen photo viewer overlay
            {move || {
                selected_photo.get().map(|photo| {
                    let photo_id = photo.id.clone();
                    let full_url = photo.data_url.clone();
                    view! {
                        <div
                            class="fixed inset-0 bg-black/90 z-50 flex flex-col items-center justify-center"
                            on:click=move |_| selected_photo.set(None)
                        >
                            <img src=full_url class="max-w-full max-h-[80vh] object-contain" />
                            <div class="flex gap-4 mt-4">
                                {if editable {
                                    let pid = photo_id.clone();
                                    Some(view! {
                                        <button
                                            class="px-4 py-2 bg-red-600 text-white rounded-lg text-sm"
                                            on:click=move |ev| {
                                                ev.stop_propagation();
                                                let pid = pid.clone();
                                                leptos::task::spawn_local(async move {
                                                    let _ = photo_store::delete_photo(&pid).await;
                                                    selected_photo.set(None);
                                                    refresh.update(|v| *v += 1);
                                                });
                                            }
                                        >
                                            {move || i18n.t("photo.delete")}
                                        </button>
                                    })
                                } else {
                                    None
                                }}
                                <button
                                    class="px-4 py-2 bg-white/20 text-white rounded-lg text-sm"
                                    on:click=move |ev| {
                                        ev.stop_propagation();
                                        selected_photo.set(None);
                                    }
                                >
                                    {move || i18n.t("common.close")}
                                </button>
                            </div>
                        </div>
                    }
                })
            }}
        </div>
    }
}
