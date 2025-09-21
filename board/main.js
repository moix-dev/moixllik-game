'use strict';

const mainMenu = document.getElementById('main-menu');
const barTop = document.getElementById('bar-top');
const barBottom = document.getElementById('bar-bottom');
const buttons = document.querySelectorAll('.parents button');

// Config
var parentState = 0;
var sunState = 0;
var moonState = 0;

// General
function load() {
    const mode = location.hash.slice(1);
    if (!mode) return;
    mainMenu.classList.add('none');
    barTop.classList.remove('none');

    barTop.querySelector('h4').innerHTML = mode.toUpperCase();

    // board hover
    addEventHover();
}

// UI
barBottom.addEventListener('click', event => {
    event.target.classList.toggle('none');
});

buttons.forEach(btn => {
    btn.addEventListener('dblclick', event => {
        const name = event.target.getAttribute('data-name');
        const max = 3;
        let state = 0;
        if (name == 'sun') {
            sunState = (sunState + 1) % (max + 1);
            state = sunState;
        } else if (name == 'moon') {
            moonState = (moonState + 1) % (max + 1);
            state = moonState;
        }
        const angle = state * -90;
        btn.style.transform = `rotate(${angle}deg)`;
    });

    btn.addEventListener('click', event => {
        const name = event.target.getAttribute('data-name');
        if (name == 'sun') parentState = 1;
        else if (name == 'moon') parentState = -1;
    });
});

window.onhashchange = load;
load();