<template>
  <Card>
    <CardContent class="space-y-5 mt-5">
      <div class="flex space-y-1">
        <Label class="flex items-center">VisableColor</Label>
        <el-color-picker v-model="config.visual.visableColor" />
      </div>
      <div class="flex space-y-1">
        <Label class="flex items-center">InVisableColor</Label>
        <el-color-picker v-model="config.visual.inVisableColor" />
      </div>
      <Accordion type="multiple" class="w-full" collapsible>
        <AccordionItem value="PlayerGlow">
          <AccordionTrigger>PlayerGlow</AccordionTrigger>
          <AccordionContent>
            <div class="flex mt-2">
              <Label class="flex items-center w-[100px]">Enable</Label>
              <Checkbox v-model:checked="config.visual.playerGlow.enable" />
            </div>
            <div class="flex mt-2">
              <Label class="flex items-center w-[100px]">Delay</Label>
              <Input type="number" class="w-30" v-model="config.visual.playerGlow.delay" />
            </div>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="ItemGlow">
          <AccordionTrigger>ItemGlow</AccordionTrigger>
          <AccordionContent>
            <div class="flex mt-2">
              <Label class="flex items-center w-[100px]">Enable</Label>
              <Checkbox v-model:checked="config.visual.itemGlow.enable" />
            </div>
            <div class="flex mt-2">
              <Label class="flex items-center w-[100px]">Delay</Label>
              <Input type="number" class="w-30" v-model="config.visual.itemGlow.delay" />
            </div>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="PlayerEsp">
          <AccordionTrigger>PlayerEsp</AccordionTrigger>
          <AccordionContent>
            <div class="flex mt-2">
              <Label class="flex items-center w-[100px]">Enable</Label>
              <Checkbox v-model:checked="config.visual.playerEsp.enable" />
            </div>
            <div class="flex mt-2">
              <Label class="flex items-center w-[100px]">Distance</Label>
              <Input type="number" class="w-30" v-model="config.visual.playerEsp.distance" />
            </div>
            <div class="flex mt-2">
              <Label class="flex items-center w-[100px]">Delay</Label>
              <Input type="number" class="w-30" v-model="config.visual.playerEsp.delay" />
            </div>
          </AccordionContent>
        </AccordionItem>
      </Accordion>
    </CardContent>
  </Card>
</template>

<script lang="ts" setup>
import 'element-plus/theme-chalk/el-popper.css'
import { Card, CardContent } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Checkbox } from '@/components/ui/checkbox'
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger
} from '@/components/ui/accordion'

import { storeToRefs } from 'pinia'
import { useConfigStore } from '@/store/Config.ts'

const configStore = useConfigStore()
const { config } = storeToRefs(configStore)
</script>

<style lang="scss">
.el-color-picker {
  margin-left: 20px;
}
</style>
