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
    // document.getElementById("header").classList.toggle('bigheader');
});

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