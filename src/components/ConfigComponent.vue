<template>
  <Card>
    <CardContent class="space-y-2 mt-5">
      <div class="flex items-center space-x-2">
        <Switch id="dark-mode" @update:checked="toggleDark()" v-model:checked="config.dark" />
        <Label for="dark-mode">DarkMode</Label>
      </div>
    </CardContent>
    <CardFooter>
      <Button @click="saveChanges()">Save</Button>
    </CardFooter>
  </Card>
</template>

<script lang="ts" setup>
import { Button } from '@/components/ui/button'
import { Switch } from '@/components/ui/switch'
import { Card, CardContent, CardFooter } from '@/components/ui/card'
import { Label } from '@/components/ui/label'

import { ref } from 'vue'
import { useDark, useToggle } from '@vueuse/core'

const isDark = useDark()
const toggleDark = useToggle(isDark)

const config = ref({
  dark: isDark.value
})

function saveChanges() {
  console.log(config.value)
}
</script>

<style lang="scss" scoped></style>
