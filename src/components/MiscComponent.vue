<template>
  <Card>
    <CardContent class="mt-5">
      <div class="flex space-x-2">
        <Switch
          id="dark-mode"
          @update:checked="toggleDark()"
          v-model:checked="config.misc.isDark"
        />
        <Label for="dark-mode">DarkMode</Label>
      </div>
      <div class="mt-3">
        <Accordion type="multiple" class="w-full" collapsible>
          <AccordionItem value="screenSize">
            <AccordionTrigger>ScreenSize</AccordionTrigger>
            <AccordionContent>
              <div class="flex mt-2">
                <Label class="w-[100px]">Screen Width</Label>
                <Input
                  id="screenWidth"
                  type="number"
                  class="w-30 ml-5"
                  v-model="config.misc.screenSize.screenWidth"
                />
              </div>
              <div class="flex mt-2">
                <Label class="w-[100px]">Screen Height</Label>
                <Input
                  id="screenHeight"
                  type="number"
                  class="w-30 ml-5"
                  v-model="config.misc.screenSize.screenHeight"
                />
              </div>
            </AccordionContent>
          </AccordionItem>
        </Accordion>
      </div>
    </CardContent>
    <CardFooter>
      <Button @click="configStore.loadConfig()">Load</Button>
      <Button class="ml-3" @click="configStore.saveConfig()">Save</Button>
    </CardFooter>
    <CardContent>
      {{ config }}
    </CardContent>
  </Card>
</template>

<script lang="ts" setup>
import { Button } from '@/components/ui/button'
import { Switch } from '@/components/ui/switch'
import { Card, CardContent, CardFooter } from '@/components/ui/card'
import { Label } from '@/components/ui/label'
import { Input } from '@/components/ui/input'
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger
} from '@/components/ui/accordion'

import { useDark, useToggle } from '@vueuse/core'
import { storeToRefs } from 'pinia'
import { useConfigStore } from '@/store/Config.ts'

const configStore = useConfigStore()
const { config } = storeToRefs(configStore)

const toggleDark = useToggle(useDark())
</script>

<style lang="scss" scoped></style>
