//import init, { caesar_cipher } from './pkg/EncryptionTool.js';
import { startMatrixAnimation } from './matrixAnimation.js';
import { startMessageDisplay } from './messageDisplay.js';
import { setupMenuInteractions } from './menuInteractions.js';

function startRustProgram() {
    // Lösche den Text und den Cursor
    const matrixText = document.getElementById('matrixText');
    const cursor = document.getElementById('cursor');
    matrixText.textContent = '';
    cursor.style.display = 'none';

    // Zeige das Menü
    const menu = document.getElementById('menu');
    menu.style.display = 'block';
}

setupMenuInteractions('menu', 'matrixText');

startMessageDisplay('cursor', 'matrixText', [
    "Wake up, Neo...",
    "The Matrix has you...",
    "Follow the white rabbit...",
    "Knock, knock, Neo..."
], startRustProgram);

startMatrixAnimation('matrixCanvas');

async function run() {
    await init();

    // Beispiel für die Verwendung der Caesar Cipher-Funktion
    const input = prompt('Enter text to encrypt:');
    const shift = parseInt(prompt('Enter shift value:'), 10);
    const result = caesar_cipher(input, shift);
    alert(`Encrypted text: ${result}`);
}

run();