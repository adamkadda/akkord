window.addEventListener('DOMContentLoaded', () => {
    const audioContainer = document.createElement('div');
    
    // Create audio elements for semitones from -12 to 23
    for (let i = -12; i <= 23; i++) {
        const audio = document.createElement('audio');
        audio.id = i;
        audio.src = `../static/audio/${i}.wav`;
        audioContainer.appendChild(audio);
    }

    // Append the generated audio elements to the body
    document.body.appendChild(audioContainer);
});

document.addEventListener("DOMContentLoaded", function() {
    const lowerOctaveArrow = document.getElementById("lower-octave-arrow");
    const lowerOctaveDiv = document.getElementById("lower-octave");

    lowerOctaveArrow.addEventListener("click", function() {
        // Toggle the "hidden" class on the "lower-octave" div
        lowerOctaveDiv.classList.toggle("hidden");
        lowerOctaveArrow.classList.toggle("left");
        lowerOctaveArrow.classList.toggle("right")
    });
});

document.addEventListener("DOMContentLoaded", function() {
    const higherOctaveArrow = document.getElementById("higher-octave-arrow");
    const higherOctaveDiv = document.getElementById("higher-octave");

    higherOctaveArrow.addEventListener("click", function() {
        // Toggle the "hidden" class on the "lower-octave" div
        higherOctaveDiv.classList.toggle("hidden");
        higherOctaveArrow.classList.toggle("left");
        higherOctaveArrow.classList.toggle("right")
    });
});

let selected = [];

const keys = document.querySelectorAll('.key');

keys.forEach(key => {
    key.addEventListener('click', () => {
        storeNote(key);
    });
});

function playNote(key) {
    const noteAudio = document.getElementById(key.dataset.semitone).cloneNode();
    noteAudio.currentTime = 0;
    noteAudio.play();
    // console.log('played:', key.dataset.semitone);
}

function storeNote(key) {
    const note = key.dataset.semitone;

    if (selected.includes(note)) {
        selected = selected.filter(n => n !== note);
        key.classList.remove('pressed');
    } else {
        selected.push(note);
        playNote(key);
        key.classList.add('pressed');
    }
}