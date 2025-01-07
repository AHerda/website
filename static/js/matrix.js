function matrix() {
    const canvasMatrix = document.querySelector('canvas.visual');
    const ctxMatrix = canvasMatrix.getContext('2d');

    // Set canvasMatrix to full screen
    canvasMatrix.width = window.innerWidth * 0.7;
    canvasMatrix.height = canvasMatrix.width * 0.5;

    // Characters to display
    const lettersMatrix = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    const fontSize = 16;
    const columns = Math.floor(canvasMatrix.width / fontSize);
    const column_width = canvasMatrix.width / columns;

    // Array to store the y-coordinate of each column
    const drops = Array.from({ length: columns }).fill(1);

    // Draw the effect
    function drawMatrix() {
        ctxMatrix.fillStyle = "rgb(18, 18, 18)";
        ctxMatrix.fillRect(0, 0, canvasMatrix.width, canvasMatrix.height);

        ctxMatrix.fillStyle = "rgb(0, 188, 212)";
        ctxMatrix.font = `${fontSize}px monospace`;

        drops.forEach((y, x) => {
            // console.log("x: ", x, "\ty: ", y);
            const levels = 7;
            let opacity = 1;
            let i = 0;
            while (opacity > 1 / levels && y - i >= 0) {
                opacity = 1 / (i + 1);
                ctxMatrix.fillStyle = `rgba(0, 188, 212, ${opacity} + )`;
                ctxMatrix.fillText(lettersMatrix.charAt(Math.floor(Math.random() * lettersMatrix.length)), ((column_width - fontSize) / 2) + x * column_width, (y - i) * fontSize);
                i += 1;
            }

            // Reset drop to the top randomly or move it down
            if (y * fontSize > canvasMatrix.height || Math.random() > 0.999) {
            drops[x] = 0;
            }

            if (Math.random() > 0.5) {
            drops[x] += Math.random();
            }
        });

        requestAnimationFrame(drawMatrix);
    }

    drawMatrix();
}
