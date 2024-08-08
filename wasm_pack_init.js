import init, { caesar_cipher } from '../pkg/encryption_tool.js';

async function run() {
    await init();
    
    const canvas = document.getElementById('matrixCanvas');
    const context = canvas.getContext('2d');

    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    const fontSize = 16;
    const columns = canvas.width / fontSize;
    const drops = Array(columns).fill(1);

    function draw() {
        context.fillStyle = 'rgba(0, 0, 0, 0.05)';
        context.fillRect(0, 0, canvas.width, canvas.height);

        context.fillStyle = '#0F0';
        context.font = `${fontSize}px monospace`;

        for (let i = 0; i < drops.length; i++) {
            const text = String.fromCharCode(Math.floor(Math.random() * 128));
            context.fillText(text, i * fontSize, drops[i] * fontSize);

            if (drops[i] * fontSize > canvas.height && Math.random() > 0.975) {
                drops[i] = 0;
            }
            drops[i]++;
        }
    }

    setInterval(draw, 33);
}

run();

const textElement = document.getElementById('text');
const inputField = document.getElementById('inputField');

let messages = [
    "Wake up, Neo...",
    "The Matrix has you...",
    "Follow the white rabbit.",
    "Knock, knock, Neo."
];

function displayMessage() {
    if (messages.length > 0) {
        const message = messages.shift();
        textElement.innerHTML += `<p>${message}</p>`;
    }
}

inputField.addEventListener('keydown', function(event) {
    if (event.key === 'Enter') {
        const userInput = inputField.value;
        inputField.value = '';
        textElement.innerHTML += `<p>${userInput}</p>`;
        
        const encryptedText = caesar_cipher(userInput, 3);
        messages.push(`Encrypted text: ${encryptedText}`);
        displayMessage();
    }
});

displayMessage();
