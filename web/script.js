function generateBoxes() {
    var wordLength = document.getElementById("word-length-input").value;
    var boxContainer = document.getElementById("word-box-container");
    boxContainer.innerHTML = "";

    // dynamically update the number of boxes when the lenght is updated
    for (var i = 0; i < wordLength; i++) {
        var input = document.createElement("input");
        input.className = "word-box";
        input.type = "text";
        input.maxLength = 1;
        input.addEventListener("input", moveCursor);

        boxContainer.appendChild(input);
    }
}

// move the cursor to the next box after a letter is entered
// move one box back in case backspace is pressed
function moveCursor(event) {
    var input = event.target;
    var keyCode = event.keyCode || event.which;

    if (keyCode === 8 && input.value.length === 0) {
        var previousInput = input.previousElementSibling;
        if (previousInput !== null) {
            previousInput.value = "";
            previousInput.focus();
        }
        else {
            var allInputs = document.getElementsByClassName("word-box");
            allInputs[allInputs.length - 1].focus();
        }
    }
    else if (input.value.length === 1) {
        var nextInput = input.nextElementSibling;
        if (nextInput !== null) {
            nextInput.focus();
        }
    }
}

// boxes from the default value
generateBoxes();