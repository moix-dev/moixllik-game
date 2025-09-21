'use strict';

const sunColor = '#FFF';
const moonColor = '#000';
const boardColor = 'peru';
const lineColor = '#222';
const slotColor = '#0003';
const hoverColor = '#0007';

const canvas = document.getElementById('board');
const ctx = canvas.getContext('2d');

// Config
var box = 0;
var mX = 0;
var mY = 0;
var boardImage = false;
var sunImages = [];
var moonImages = [];

function draw() {
    box = canvas.offsetWidth / 7;

    canvas.style.opacity = '0';
    drawParent(sunImages, sunColor);
    drawParent(moonImages, moonColor);
    drawBoard();
    canvas.style.opacity = '1';
}

// Parent
function drawParent(parent, color) {
    const pos = [
        [0.5, 0.2], // top
        [0.2, 0.5], // left
        [0.5, 0.8], // bottom
        [0.8, 0.5], // right
    ];
    canvas.width = box;
    canvas.height = box;

    for (var p of pos) {
        ctx.beginPath();
        ctx.arc(box / 2, box / 2, box / 2, 0, Math.PI * 2);
        ctx.arc(box * p[0], box * p[1], box / 6, 0, Math.PI * 2);
        ctx.fillStyle = color;
        ctx.fill('evenodd');

        const img = new Image();
        img.src = canvas.toDataURL();
        parent.push(img);
        ctx.clearRect(0, 0, canvas.width, canvas.height);
    }
}

// Board
function drawBoard() {
    const b = box;

    canvas.width = canvas.offsetWidth;
    canvas.height = canvas.offsetWidth;

    ctx.fillStyle = lineColor;
    for (var x = 1; x < 7; x++) {
        ctx.fillRect(x * b, 0, 0.01 * b, 7 * b);
        ctx.fillRect(0, x * b, 7 * b, 0.01 * b);
    }

    ctx.fillStyle = slotColor;
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

    boardImage = new Image();
    boardImage.src = canvas.toDataURL();
}

function boardClear() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.drawImage(boardImage, 0, 0, canvas.width, canvas.height);
}

function addEventHover() {
    canvas.addEventListener('mousemove', (event) => {
        const rect = canvas.getBoundingClientRect();
        mX = Math.floor((event.clientX - rect.left) / box);
        mY = Math.floor((event.clientY - rect.top) / box);

        boardClear();
        ctx.fillStyle = hoverColor;
        ctx.fillRect(mX * box, mY * box, box, box);
    });
}

window.onresize = draw;
draw();