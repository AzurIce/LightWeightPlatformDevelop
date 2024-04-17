const SETTING = {
  width: 480,
  height: 700,
};

class Game {
  constructor(options) {
    console.log(`contructing game engine on caontainer ${options.container}...`)
    this.container = document.getElementById(options.container);
  }

  function() {
    
  }
}

(function () {
  var appEl = document.getElementById("app");
  function setSize() {
    // console.log("window.innerWidth", window.innerWidth, "window.innerHeight", window.innerHeight)
    const widthRatio = window.innerWidth / SETTING.width;
    const heightRatio = window.innerHeight / SETTING.height;
    appEl.style.transform = `scale(${widthRatio < heightRatio ? widthRatio : heightRatio})`;
  }
  setSize();
  window.onresize = setSize;

  const game = new Game({
    container: "app",
  });
})();

