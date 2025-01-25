# Flood Fill

Can be found on LeetCode.

You are given image by an `m x n` grid of integers `image`,
where `image[i][j]` represents the pixel value of the image.
You are also given three integers, `sr`, `sc`, and `color`.
The task is to perform a **flood fill** on the image,
starting from pixel `image[sr][sc]`.

Steps:

1. change the first color to `color`.
2. perform the same process for each pixel that is directly adjacent and share the same color as the starting pixel.
3. Repeat process by checking neighboring pixels of the updated pixel and modify the original color of the starting pixel.
4. The process stops when there are no more adjacent pixels of the original color to update.

Return the modified image after performing the flood fill.

Constraints are that the starting pixels must be within the bounds of the image.
