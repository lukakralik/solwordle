var rowCounter = 1;

function generateBoxes() {
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
        colorInput.value = "#A4AEC4";

        // create a pair of inputs
        var boxWrapper = document.createElement("div");
        boxWrapper.className = "word-box-wrapper";
        boxWrapper.appendChild(input);
        boxWrapper.appendChild(colorInput);

        boxContainer.appendChild(boxWrapper);
    }
}

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

function checkGuess() {
    var wordLength = document.getElementById("word-length-input").value;
    var boxContainer = document.getElementById("word-box-container");

    var lineDiv = document.createElement("div");
    lineDiv.className = "line";

    // Generate the input boxes and color pickers
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
        colorInput.value = "#A4AEC4";

        var boxWrapper = document.createElement("div");
        boxWrapper.className = "word-box-wrapper";
        boxWrapper.appendChild(input);
        boxWrapper.appendChild(colorInput);
        boxWrapper.classList.add("input-pair");

        lineDiv.appendChild(boxWrapper);
    }

    boxContainer.appendChild(lineDiv);

    // Focus on the first input of the new line
    var firstInput = lineDiv.querySelector(".word-box");
    firstInput.focus();

    // Reset the input values and color pickers for the next line
    var wordBoxes = lineDiv.querySelectorAll(".word-box");
    var colorPickers = lineDiv.querySelectorAll(".color-picker");

    Array.from(wordBoxes).forEach(input => (input.value = ""));
    Array.from(colorPickers).forEach(
        colorPicker => (colorPicker.value = "#A4AEC4")
    );

    // TODO: Process the guessed word and colors here
    processGuess(wordBoxes, colorPickers);
}   

// Call the generateBoxes() function initially
generateBoxes();