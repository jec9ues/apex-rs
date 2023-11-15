import { createApp } from 'vue'
import { createPinia } from 'pinia'
import ElementPlus from 'element-plus'
import 'element-plus/theme-chalk/dark/css-vars.css'
import App from './App.vue'

const app = createApp(App)

app.directive('drag', {
  mounted(el: HTMLElement) {
    el.onmousedown = function (event) {
      var disx = event.pageX - el.offsetLeft
      var disy = event.pageY - el.offsetTop
      document.onmousemove = function (event) {
        if (
          !(
            event.pageX - disx + el.offsetWidth >= document.body.offsetWidth - 20 ||
            event.pageX - disx <= 20
          )
        ) {
          el.style.left = event.pageX - disx + 'px'
        }

        if (
          !(
            event.pageY - disy <= 20 ||
            event.pageY - disy + el.offsetHeight >= document.body.offsetHeight - 20
          )
        ) {
          el.style.top = event.pageY - disy + 'px'
        }
      }

      document.onmouseup = function () {
        document.onmousemove = document.onmouseup = null
      }
    }

    window.onresize = function () {
      el.style.left = '50px'
      el.style.top = '50px'
    }
  }
})

app.use(ElementPlus)
app.use(createPinia())
app.mount('#app')
