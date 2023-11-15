import { defineStore } from 'pinia'
import { useDark } from '@vueuse/core'

export const useConfigStore = defineStore('Config', {
  state: () => ({
    config: {
      misc: {
        isDark: useDark().value,
        screenSize: {
          screenWidth: 0,
          screenHeight: 0
        }
      },
      visual: {
        visableColor: '#000000',
        inVisableColor: '#000000',
        playerGlow: {
          enable: false,
          delay: 1
        },
        itemGlow: {
          enable: false,
          delay: 1
        },
        playerEsp: {
          enable: false,
          distance: 1,
          delay: 1
        }
      }
    }
  }),
  getters: {},
  actions: {
    loadConfig() {
      console.log(this.config)
    },
    saveConfig() {
      console.log(this.config)
    }
  }
})
