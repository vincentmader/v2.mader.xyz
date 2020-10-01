const canvas = document.getElementById("canvas1");
const ctx = canvas.getContext("2d");

const keys = [];

const player = {
  x: canvas.width / 2,
  y: canvas.height / 2,
  width: 32,
  height: 48,
  frameX: 0,
  frameY: 0,
  speed: 10,
  moving: false,
};

var playerSprite = new Image();
playerSprite.src = "/static/media/sprites/sallah.png";
const background = new Image();
background.src = "/static/media/background.png";

function drawSprite(img, sX, sY, sW, sH, dX, dY, dW, dH) {
  ctx.drawImage(img, sX, sY, sW, sH, dX, dY, dW, dH);
}

window.addEventListener("keydown", function (e) {
  if (keys[37] || keys[38] || keys[39] || keys[40]) {
    player.moving = true;
  }
  keys[e.keyCode] = true;
});
window.addEventListener("keyup", function (e) {
  if (keys[37] || keys[38] || keys[39] || keys[40]) {
    player.moving = false;
  }
  delete keys[e.keyCode];
});
window.addEventListener("click", function (e) {
  const characters = [
    "darthvader",
    "sallah",
    "stormtrooper",
    "henryjones",
    "obiwan1",
  ];
  var random_char = characters[Math.floor(Math.random() * characters.length)];
  playerSprite.src = "/static/media/sprites/" + random_char + ".png";
});

function movePlayer() {
  if (keys[38] && player.y > 20) {
    player.y -= player.speed;
    player.frameY = 3;
    player.moving = true;
  }
  if (keys[37] && player.x > 0) {
    player.x -= player.speed;
    player.frameY = 1;
    player.moving = true;
  }
  if (keys[40] && player.y < canvas.height - player.height) {
    player.y += player.speed;
    player.frameY = 0;
    player.moving = true;
  }
  if (keys[39] && player.x < canvas.width - player.width) {
    player.x += player.speed;
    player.frameY = 2;
    player.moving = true;
  }
}

function handlePlayerFrame() {
  if (player.frameX < 3 && player.moving == true) player.frameX++;
  else player.frameX = 0;
}

let fps, fpsInterval, startTime, now, then, elapsed;
function startAnimating(fps) {
  fpsInterval = 1000 / fps;
  then = Date.now();
  startTime = then;
  animate();
}
function animate() {
  requestAnimationFrame(animate);
  now = Date.now();
  elapsed = now - then;
  if (elapsed > fpsInterval) {
    then = now - (elapsed % fpsInterval);
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.drawImage(background, 0, 0, canvas.width, canvas.height);
    drawSprite(
      playerSprite,
      player.width * player.frameX,
      player.height * player.frameY,
      player.width,
      player.height,
      player.x,
      player.y,
      1.5 * player.width,
      player.height
    );
    movePlayer();
    handlePlayerFrame();
    requestAnimationFrame(animate);
  }
}
startAnimating(15);
