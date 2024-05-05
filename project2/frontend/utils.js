export async function load_image_bitmap(name) {
    const image_response = await fetch(`./images/${name}`);
    const image_blob = await image_response.blob();
    return await createImageBitmap(image_blob);
}