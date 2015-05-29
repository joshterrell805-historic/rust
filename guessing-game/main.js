// I don't think this is more readable.
// it's also not much smaller and ends up being the same size if I keep the same
// curly styles
var readline = require('readline');
var rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

function randomInt (low, high) {
  return Math.floor(Math.random() * (high - low) + low);
}

var number = randomInt(1, 101);
console.log("Guessing Game");
read_number(on_read);

function on_read(err, guess) {
  if (err) {
    console.log(err.message());
  } else {
    if (guess < number) {
      console.log("Too small.");
    } else if (guess > number) {
      console.log("too big.");
    } else {
      console.log("You win!");
      rl.close();
      return;
    }
  }
  read_number(on_read);
}

function read_number(callback)
{
  rl.question("Enter a number: ", function(input) {
    var num = parseInt(input);
    if (isNaN(num)) {
      callback(new Error('Not a number!'));
    } else {
      callback(null, num);
    }
  });
}
