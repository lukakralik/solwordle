function generateBoxes() {
    var wordLength = document.getElementById("word-length-input").value;
    var boxContainer = document.getElementById("word-box-container");
    boxContainer.innerHTML = "";

    // dynamically update the number of boxes when the length is updated
    for (var i = 0; i < wordLength; i++) {
        var input = document.createElement("input");
        input.className = "word-box";
        input.id = "box" + i;
        input.type = "text";
        input.maxLength = 2;
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
        } else {
            var allInputs = document.getElementsByClassName("word-box");
            allInputs[allInputs.length - 1].focus();
        }
    } else if (input.value.length === 2) {
        // Capitalize the letter
        input.value = input.value.toUpperCase();

        var nextInput = input.nextElementSibling;
        if (nextInput !== null) {
            nextInput.focus();
        }
    }
}

// ran each time the check button is pressed, process the existing results
// go to the next line and continue
async function checkGuess() {
    var wordLength = document.getElementById("word-length-input").value;
    var slovo = "";
    var barvy = "";
    for(i = 0; i < wordLength; i++) 
    {
        slovo = slovo + Array.from(document.getElementById("box" + i).value)[0];
        barvy = barvy + Array.from(document.getElementById("box" + i).value)[1];
    }
    console.log(slovo);
    console.log(barvy);
    data = {"slovo": slovo, "barvy": barvy};

    for(i = 0; i < wordLength; i++)
    {
        document.getElementById("box" + i).value = "";
    }
    
    const response = await fetch("www.solwordle.org", {
        method: "POST", // *GET, POST, PUT, DELETE, etc.,
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(data), // body data type must match "Content-Type" header
      });
}
// boxes from the default value
generateBoxes();
