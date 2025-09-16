const canvas = document.getElementById('board');
const ctx = canvas.getContext('2d');

function draw() {
    const b = canvas.offsetWidth / 7;

    canvas.width = canvas.offsetWidth;
    canvas.height = canvas.offsetWidth;

    ctx.fillStyle = '#000';
    for (var x = 1; x < 7; x++) {
        ctx.fillRect(x * b, 0, 0.01 * b, 7 * b);
        ctx.fillRect(0, x * b, 7 * b, 0.01 * b);
    }

    ctx.fillStyle = '#0003';
    // Islas
    ctx.fillRect(0, 0, b, b);
    ctx.fillRect(0, 6 * b, b, b);
    ctx.fillRect(6 * b, 0, b, b);
    ctx.fillRect(6 * b, 6 * b, b, b);
    // Continente
    ctx.fillRect(0, 3 * b, 7 * b, b);
    ctx.fillRect(3 * b, 0, b, 7 * b);
    ctx.fillRect(1 * b, 2 * b, 5 * b, 3 * b);
    ctx.fillRect(2 * b, 1 * b, 3 * b, 5 * b);
}

draw();
window.onresize = draw;