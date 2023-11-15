<template>
  <div v-drag class="draggable w-[500px]">
    <Tabs default-value="AimBot" class="w-[100%]">
      <TabsList class="grid w-full grid-cols-4">
        <TabsTrigger value="AimBot"> AimBot </TabsTrigger>
        <TabsTrigger value="Visual"> Visual </TabsTrigger>
        <TabsTrigger value="Misc"> Misc </TabsTrigger>
        <TabsTrigger value="Debug"> Debug </TabsTrigger>
      </TabsList>
      <TabsContent value="AimBot">
        <AimBotComponet />
      </TabsContent>
      <TabsContent value="Visual">
        <VisualComponet />
      </TabsContent>
      <TabsContent value="Misc">
        <MiscComponent />
      </TabsContent>
      <TabsContent value="Debug">
        <DebugComponet />
      </TabsContent>
    </Tabs>
  </div>
</template>

<script lang="ts" setup>
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'

import AimBotComponet from '@/components/AimBotComponent.vue'
import VisualComponet from '@/components/VisualComponent.vue'
import MiscComponent from '@/components/MiscComponent.vue'
import DebugComponet from '@/components/DebugComponent.vue'

import { useDark } from '@vueuse/core'
useDark()
</script>

<style lang="scss">
@import url('@/assets/css/main.css');

body {
  width: 100vw;
  height: 100vh;
  overflow: hidden;

  #app {
    width: 100%;
    height: 100%;

    .draggable {
      position: absolute;
      left: 50px;
      top: 50px;
    }
  }

  user-select: none;
}
</style>
