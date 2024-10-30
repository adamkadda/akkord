/* =========================================================
                        GLOBAL
========================================================= */

const audioElements = document.querySelectorAll("audio");
const keys = document.querySelectorAll('.key');

const audioPipelines = {}

let selected = [];
const MAX_NOTES = 9;

/* =========================================================
                        NAVIGATION
========================================================= */

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

/* =========================================================
                        CONTROLS
========================================================= */

const muteButton = document.getElementById('mute');
let isMuted = false;

function muteAllPipelines(mute = true) {
    Object.values(audioPipelines).forEach((pipeline) => {
        const { gainNode } = pipeline;
        gainNode.gain.value = mute ? 0 : 1;
    });
}

document.getElementById('mute').addEventListener('click', () => {
    
    isMuted = !isMuted;
    
    muteAllPipelines(isMuted);

    muteButton.textContent = isMuted ? 'unmute' : 'mute';
});

function clearKeyboard() {
    selected.forEach(note => {
        const key = document.querySelector(`[data-semitone="${note}"]`);
        key.classList.remove('pressed');
    });
    selected.length = 0;
}

function resetSelection() {
    clearKeyboard();
    document.getElementById('result').innerHTML = '';
}

document.getElementById('reset').addEventListener('click', resetSelection);

/* =========================================================
                        PIANO
========================================================= */

function initAudioPipeline(audioElement) {
    const audioContext = new (window.AudioContext || window.webkitAudioContext)();
    const sourceNode = audioContext.createMediaElementSource(audioElement);
    const gainNode = audioContext.createGain();

    sourceNode.connect(gainNode).connect(audioContext.destination);

    audioPipelines[audioElement.id] = {
        audioElement: audioElement,
        audioContext: audioContext,
        gainNode: gainNode,
    }
}

function prepareNotes(audioElements) {
    audioElements.forEach(audioElement => initAudioPipeline(audioElement));
}

prepareNotes(audioElements);

function playNote(key) {
    const pipeline = audioPipelines[key.dataset.semitone];
    if (pipeline) {
        const { audioElement } = pipeline;
        audioElement.currentTime = 0;
        audioElement.play();
    }
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
        if (selected.length == MAX_NOTES){
            resetSelection();
            alert("That's too many notes, sorry!");
            return;
        }
        selected.push(note);
        playNote(key);
        key.classList.add('pressed');
    }
}

keys.forEach(key => {
    key.addEventListener('click', () => {
        storeNote(key);
    });
});

/* =========================================================
                        CHORD PLAYBACK
========================================================= */

function storeNotesSequentially(noteElements) {
    noteElements.forEach((noteElement, index) => {
        setTimeout(() => {
            storeNote(noteElement);
        }, (index * 100));
    });
}

function playChord(button) {
    const notes = button.getAttribute('data-notes')
        .split(',')
        .map(note => note.trim());

    const noteElements = notes.map(note => {
        return document.querySelector(`div[data-semitone="${note}"]`)
    });

    clearKeyboard();

    storeNotesSequentially(noteElements);
}

function prepareResult() {
    const container = document.getElementById("result");

    container.addEventListener("click", function(event) {
        let button = event.target.closest('.chord');
        if (button) {
            playChord(button);
        }
    });
}

prepareResult();