const WIDTH = 240;
const HEIGHT = 136;

const rects = [];

function addRect() {
  rects.push({
      x: Math.random() * WIDTH,
      y: Math.random() * HEIGHT,
      w: 16,
      h: 16,
      dx: Math.random() * 2 - 1,
      dy: Math.random() * 2 - 1,
      color: Math.random() * 0xffffff,

      update: function() {
          this.x += this.dx
          this.y += this.dy
          if (this.x < 0) { this.dx = Math.abs(this.dx) }
          if (this.x + this.w > WIDTH) { this.dx = -Math.abs(this.dx) }
          if (this.y < 0) { this.dy = Math.abs(this.dy) }
          if (this.y + this.h > HEIGHT) { this.dy = -Math.abs(this.dy) }
      }
  })
}

function update() {
  cls();

  const framerate = getFramerate();
  // print("default values");
  // print("Hello from Boa!", 16, 16, 0xff0000);
  // print("Alpha transparency", 16, 32, 0x40ff0000);

  if (framerate > 50) {
    for (let i = 0; i < 4; i++) {
      addRect();
    }
  }

  for (const rect of rects) {
    rect.update();
    drawRectFill(rect.x, rect.y, 16, 16, rect.color);
  }

  print("FPS: " + framerate, 0, 0, 0xffffff);
  print("Rects: " + rects.length, 0, 10, 0xffffff);
}
