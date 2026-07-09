pub fn draw_bitmap(
    x: usize,
    y: usize,
    bitmap: &[bool],
    width: usize,
    height: usize,
    frame: &mut [u8],
    frame_width: usize,
) {
    for j in 0..height {
        for i in 0..width {
            let idx = j * width + i;
            if idx < bitmap.len() && bitmap[idx] {
                let fx = x + i;
                let fy = y + j;
                let fi = (fy * frame_width + fx) * 4;
                if fi + 3 < frame.len() {
                    frame[fi..fi + 4].copy_from_slice(&[255, 255, 255, 255]);
                }
            }
        }
    }
}
