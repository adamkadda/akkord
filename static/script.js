document.querySelector("nav").addEventListener("click", function() {
    const links = document.querySelector(".links");
    if (links.style.display === "none" || links.style.display === "") {
        links.style.display = "block";
    } else {
        links.style.display = "none";
    }
});

document.getElementById("nav-icon").addEventListener("click", function() {
    this.classList.toggle('open');
});

document.getElementById('mute').addEventListener('click', () => {
    const audioElements = document.querySelectorAll("audio");

    audioElements.forEach(audio => {
        audio.muted = !audio.muted;
    });
    
    const muteButton = document.getElementById('mute');
    muteButton.textContent = audioElements[0].muted ? 'unmute' : 'mute';
});


document.getElementById('reset').addEventListener('click', () => {
    selected.forEach(note => {
        const key = document.querySelector(`[data-semitone="${note}"]`);
        key.classList.remove('pressed');
    });

    selected.length = 0;
    
    document.getElementById('result').innerHTML = '';
});

let selected = [];

const keys = document.querySelectorAll('.key');

keys.forEach(key => {
    key.addEventListener('click', () => {
        storeNote(key);
    });
});

function playNote(key) {
    const original = document.getElementById(key.dataset.semitone);
    const copy = original.cloneNode(); // does not copy dynamic properties (e.g. muted)
    
    copy.muted = original.muted; // pass the original's state for its 'muted' property
    copy.currentTime = 0;
    copy.play();

    console.log('Playing note:', key.dataset.semitone);
}

function storeNote(key) {
    const note = key.dataset.semitone;

    if (selected.includes(note)) {
        selected = selected.filter(n => n !== note);
        key.classList.remove('pressed');
        if (selected.length == 0) {
            document.getElementById('result').innerHTML = '';
        }
    } else {
        selected.push(note);
        playNote(key);
        key.classList.add('pressed');
    }
}