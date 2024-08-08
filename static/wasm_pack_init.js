const canvas = document.getElementById('matrixCanvas');
const ctx = canvas.getContext('2d');

canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

const chars = '0123456789/*-+/<>?;:[]~!@#$%^&*()+=abcdefghijklmnopqrstuvwxyz';
const fontSize = 16;
const columns = canvas.width / fontSize;
const drops = Array.from({ length: columns }, () => 0);

// Initialisiere Skalierungsfaktoren
const scales = Array.from({ length: columns }, () => 1);
const scaleDirection = Array.from({ length: columns }, () => 1);

function draw() {
    ctx.fillStyle = 'rgba(0, 0, 0, 0.05)';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    drops.forEach((drop, x) => {
        const text = chars[Math.floor(Math.random() * chars.length)];
        
        // Dynamische Skalierung
        let scale = scales[x];
        scale += scaleDirection[x] * 0.05;
        if (scale > 1.8 || scale < 1) {
            scaleDirection[x] *= -1; // Ändere die Richtung der Skalierung
        }
        scales[x] = scale;

        ctx.font = `${fontSize * scale}px monospace`;
        ctx.fillStyle = 'green';
        ctx.fillText(text, x * fontSize, drop * fontSize);

        if (drop * fontSize > canvas.height || Math.random() > 0.97) {
            drops[x] = 0;
        } else {
            drops[x]++;
        }
    });
}

// Blinke den Cursor 5 Mal und zeige dann den Text an
let blinkCount = 0;
const cursor = document.getElementById('cursor');
const matrixText = document.getElementById('matrixText');
const messages = ["Wake up, Neo...", "The Matrix has you...", "Follow the white rabbit...", "Knock, knock, Neo..."];
let currentMessageIndex = 0;

function blinkCursor() {
    cursor.style.opacity = cursor.style.opacity === '0' ? '1' : '0';
    blinkCount++;
    if (blinkCount < 10) { // 10 Wechsel = 5 Mal blinken
        setTimeout(blinkCursor, 500);
    } else {
        cursor.style.display = 'none';
        showMessage();
    }
}

function showMessage() {
    if (currentMessageIndex < messages.length) {
        const message = messages[currentMessageIndex];
        let index = 0;
        matrixText.textContent = ''; // Lösche den vorherigen Text

        function typeNextCharacter() {
            if (index < message.length) {
                matrixText.textContent += message[index++];
                setTimeout(typeNextCharacter, 150);
            } else {
                currentMessageIndex++;
                setTimeout(() => {
                    cursor.style.display = 'inline-block';
                    blinkCursor(); // Blinke den Cursor erneut vor der nächsten Nachricht
                }, 1000); // Eine Sekunde warten, bevor der nächste Text angezeigt wird
            }
        }

        typeNextCharacter();
    } else {
        // Alle Nachrichten angezeigt, jetzt das eigentliche Programm starten
        startRustProgram();
    }
}

function startRustProgram() {
    // Lösche den Text und den Cursor
    matrixText.textContent = '';
    cursor.style.display = 'none';

    // Zeige den Text zum Starten des eigentlichen Programms
    matrixText.textContent = 'Encryptet Text:';

    // Jetzt kann der Benutzer den Text eingeben und das Rust-Programm übernimmt
    const inputField = document.getElementById('inputField');
    inputField.style.display = 'inline-block';
    inputField.focus();
}

setTimeout(blinkCursor, 500); // Startet den Blinkeffekt

setInterval(draw, 80); // 80ms Verzögerung zwischen den Frames (12.5fps)
