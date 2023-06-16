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
    var wordLength = +document.getElementById("word-length-input").value;
    var word = "";
    var colors = "";
	var language = document.getElementById("lang").value;
    for(i = 0; i < wordLength; i++)  {
        word = word + Array.from(document.getElementById("box" + i).value)[0];
        colors = colors + Array.from(document.getElementById("box" + i).value)[1];
    }
    console.log(word);
    console.log(colors);
    data = {
		"word": word,
		"colors": colors,
		"length": wordLength,
		"lang": language
	};

    for(i = 0; i < wordLength; i++) {
        document.getElementById("box" + i).value = "";
    }

    const response = await fetch("http://localhost:8000/words", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
      });
      
      if (response.ok) {
        const responseData = await response.json();
        const words = responseData.words;
        console.log(words);
      
        var result = document.getElementById("answersparagraph");
        result.innerHTML = words.join(", ");
      } else {
        throw new Error('Error:', response.status);
      }
}

// boxes from the default value
generateBoxes();
