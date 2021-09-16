var canvas = document.getElementById("canvas");
var ctx = canvas.getContext("2d");
var W = canvas.getBoundingClientRect().width;
var H = W;
canvas.width = W;
canvas.height = W;

const keys = [];

const player = {
  character: "sallah",
  x: 0,
  y: 0,
  z: 0,
  v_z: 0,
  width: 32,
  height: 48,
  frameX: 0,
  frameY: 0,
  speed: 10,
  moving: false,
  jumping: false,
  jump: function () {
    player.v_z = 10;
    player.jumping = true;
  },
  applyGravity: function () {
    player.v_z -= 1.5;
    if (player.z + player.v_z >= 0) {
      player.z += player.v_z;
    } else {
      player.z = 0;
      player.jumping = false;
    }
  },
};

var playerSprite = new Image();
playerSprite.src = "/static/media/sprites/" + player.character + ".png";
const background = new Image();
background.src = "/static/media/background.png";
const shadow = new Image();
shadow.src = "/static/media/shadow.png";

function drawSprite(img, sX, sY, sW, sH, dX, dY, dW, dH) {
  ctx.drawImage(img, sX, sY, sW, sH, dX, dY, dW, dH);
}

window.addEventListener("keydown", function (e) {
  keys[e.keyCode] = true;
  if (keys[37] || keys[38] || keys[39] || keys[40]) {
    player.moving = true;
  }
  if (keys[32]) {
    player.jump();
  }
});
window.addEventListener("keyup", function (e) {
  if (
    keys[37] ||
    keys[38] ||
    keys[39] ||
    keys[40] ||
    keys[75] ||
    keys[72] ||
    keys[74] ||
    keys[76]
  ) {
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
  for (let idx = 0; idx < characters.length; idx++) {
    if (player.character == characters[idx]) {
      if (idx + 1 < characters.length) {
        character = characters[idx + 1];
      } else {
        character = characters[0];
      }
    }
  }
  player.character = character;
  playerSprite.src = "/static/media/sprites/" + character + ".png";
});

function movePlayer() {
  if ((keys[38] || keys[75]) && player.y < canvas.height / 2 - 20) {
    player.y += player.speed;
    player.frameY = 3;
    player.moving = true;
  }
  if ((keys[37] || keys[72]) && player.x > -canvas.width / 2) {
    player.x -= player.speed;
    player.frameY = 1;
    player.moving = true;
  }
  if ((keys[40] || keys[74]) && player.y > -canvas.height / 2 + player.height) {
    player.y -= player.speed;
    player.frameY = 0;
    player.moving = true;
  }
  if ((keys[39] || keys[76]) && player.x < canvas.width / 2 - player.width) {
    player.x += player.speed;
    player.frameY = 2;
    player.moving = true;
  }
}

function handlePlayerFrame() {
  if (player.frameX < 3 && player.moving == true) player.frameX++;
  else player.frameX = 0;
  if (player.jumping) player.frameX = 1;
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
      shadow,
      0,
      0,
      player.width,
      player.height,
      player.x + player.width / 4 + canvas.width / 2,
      -(player.y - 0.8 * player.height) + canvas.height / 2,
      player.width,
      player.height
    );
    drawSprite(
      playerSprite,
      player.width * player.frameX,
      player.height * player.frameY,
      player.width,
      player.height,
      player.x + canvas.width / 2,
      -(player.y + player.z) + canvas.height / 2,
      1.2 * player.width,
      player.height
    );
    movePlayer();
    player.applyGravity();
    handlePlayerFrame();
    requestAnimationFrame(animate);
  }
}
startAnimating(15);
