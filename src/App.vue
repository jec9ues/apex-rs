<template>
  <div v-drag class="draggable w-[500px]">
    <Tabs default-value="Debug" class="w-[100%]">
      <TabsList class="grid w-full grid-cols-3">
        <TabsTrigger value="Debug"> Debug </TabsTrigger>
        <TabsTrigger value="Menu"> Menu </TabsTrigger>
        <TabsTrigger value="Config"> Config </TabsTrigger>
      </TabsList>
      <TabsContent value="Debug">
        <DebugComponet />
      </TabsContent>
      <TabsContent value="Menu">
        <MenuComponet />
      </TabsContent>
      <TabsContent value="Config">
        <ConfigComponent />
      </TabsContent>
    </Tabs>
  </div>
</template>

<script lang="ts" setup>
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'

import DebugComponet from '@/components/DebugComponent.vue'
import MenuComponet from '@/components/MenuComponent.vue'
import ConfigComponent from '@/components/ConfigComponent.vue'

import { useDark } from '@vueuse/core'
useDark()
</script>

<style lang="scss">
@import url('@/assets/css/main.css');

body {
  width: 100vw;
  height: 100vh;

  #app {
    width: 100%;
    height: 100%;

    .draggable {
      position: absolute;
      left: 100px;
      top: 100px;
    }
  }

  user-select: none;
}
</style>
