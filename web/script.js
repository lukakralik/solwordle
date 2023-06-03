var rowCounter = 1;

function generateBoxes() {
    // get the word length input value
    var wordLength = document.getElementById("word-length-input").value;
    var boxContainer = document.getElementById("word-box-container");
    boxContainer.innerHTML = "";

    // dynamically update the number of boxes and color pickers when the length is updated
    for (var i = 0; i < wordLength; i++) {
        var input = document.createElement("input");
        input.className = "word-box";
        input.type = "text";
        input.maxLength = 1;
        input.addEventListener("input", moveCursor);

        // create new color input for each letter and assign gray as the default color
        var colorInput = document.createElement("input");
        colorInput.className = "color-picker";
        colorInput.type = "color";
        colorInput.setAttribute("list", "color-options");
        colorInput.value = "#a4aec4";

        // create a pair of inputs
        var boxWrapper = document.createElement("div");
        boxWrapper.className = "word-box-wrapper";
        boxWrapper.appendChild(input);
        boxWrapper.appendChild(colorInput);

        boxContainer.appendChild(boxWrapper);
    }
}

// TODO: move cursor to the next letter in the row after entering the current one
function moveCursor(event) {
    var input = event.target;
    var keyCode = event.keyCode || event.which;

    if (keyCode === 8 && input.value.length === 0) {
        var previousInput = input.previousElementSibling;
        if (previousInput !== null) {
            previousInput.value = "";
            previousInput.focus();
        } else {
            var allInputs = document.getElementsByClassName("word-box");
            allInputs[allInputs.length - 1].focus();
        }
    } else if (input.value.length === 1) {
        input.value = input.value.toUpperCase();
        var nextInput = input.nextElementSibling;
        if (nextInput !== null) {
            nextInput.focus();
        }
    }
}

// check the correctness of the last guess and generate new line
function checkGuess() {
    var wordLength = document.getElementById("word-length-input").value;
    var boxContainer = document.getElementById("word-box-container");

    // create a new line container for the current pair
    var lineDiv = document.createElement("div");
    lineDiv.className = "line";

    // generate the input boxes and color pickers
    for (var i = 0; i < wordLength; i++) {
        var input = document.createElement("input");
        input.className = "word-box";
        input.type = "text";
        input.maxLength = 1;
        input.addEventListener("input", moveCursor);

        var colorInput = document.createElement("input");
        colorInput.className = "color-picker";
        colorInput.type = "color";
        colorInput.setAttribute("list", "color-options");
        colorInput.value = "#a4aec4";

        var boxWrapper = document.createElement("div");
        boxWrapper.className = "word-box-wrapper";
        boxWrapper.appendChild(input);
        boxWrapper.appendChild(colorInput);

        lineDiv.appendChild(boxWrapper);
    }

    // insert the new line container below the current pair
    var existingLines = document.getElementsByClassName("line");
    if (existingLines.length > 0) {
        boxContainer.insertBefore(lineDiv, existingLines[existingLines.length - 1].nextSibling);
    } else {
        boxContainer.appendChild(lineDiv);
    }

    // focus on the first input of the new line
    var firstInput = lineDiv.querySelector(".word-box");
    firstInput.focus();

    // reset the input values and color pickers for the next line
    var wordBoxes = lineDiv.querySelectorAll(".word-box");
    var colorPickers = lineDiv.querySelectorAll(".color-picker");

    // TODO: find a better way to construct an array from the paired letters and colors
    Array.from(wordBoxes).forEach(input => (input.value = ""));
    Array.from(colorPickers).forEach(colorPicker => (colorPicker.value = "#a4aec4"));

    // TODO: process the guessed word and colors here
    processGuess(wordBoxes, colorPickers);
}

// call the generateBoxes() function initially
generateBoxes();