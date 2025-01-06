document.addEventListener("DOMContentLoaded", () => {
    const canvasBackground = document.querySelector('canvas.background');
    const ctxBackground = canvasBackground.getContext('2d');

    // Set canvas to full screen
    canvasBackground.width = window.innerWidth;
    canvasBackground.height = window.innerHeight;

    // Characters to display
    const letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    const fontSize = 16;
    const columns = Math.floor(canvasBackground.width / fontSize);
    const column_width = canvasBackground.width / columns;

    // Array to store the y-coordinate of each column
    const drops = Array.from({ length: columns }).fill(1);

    // Draw the effect
    function drawMatrix() {
        ctxBackground.fillStyle = "rgb(18, 18, 18)";
        ctxBackground.fillRect(0, 0, canvasBackground.width, canvasBackground.height);

        ctxBackground.font = `${fontSize}px monospace`;

        drops.forEach((y, x) => {
            // console.log("x: ", x, "\ty: ", y);
            const levels = 10;
            let opacity = 1;
            let i = 0;
            while (opacity > 1 / levels && y - i >= 0) {
                opacity = 1 / (i + 1);
                ctxBackground.fillStyle = `rgba(255, 255, 255, ${opacity / 5})`;
                ctxBackground.fillText(letters.charAt(Math.floor(Math.random() * letters.length)), ((column_width - fontSize) / 2) + x * column_width, (y - i) * fontSize);
                i += 1;
            }

            // Reset drop to the top randomly or move it down
            if (y * fontSize > canvasBackground.height || Math.random() > 0.999) {
            drops[x] = 0;
            }

            if (Math.random() > 0.5) {
            drops[x] += Math.random();
            }
        });

        requestAnimationFrame(drawMatrix);
    }

    drawMatrix();
}); // end of DOMContentLoaded
