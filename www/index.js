import { Client } from 'gallery-rs';

const client = new Client();
let updateScheduled = false;

window.GlobalJS = function() {}
window.GlobalJS.prototype.update = function() {
  if (!updateScheduled) {
    requestAnimationFrame(() => {
      client.render();
      updateScheduled = false;
    });
  }
  updateScheduled = true;
}

window.global_js = new GlobalJS();
