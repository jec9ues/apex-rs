<template>
  <Card>
    <CardContent class="space-y-2 mt-5">
      <div class="space-y-1">
        <Label for="currentFps">current fps: 325.2603</Label>
        <Label for="cursorPosX">cursor pos x: 325.2603</Label>
        <Label for="cursorPosY">cursor pos y: 325.2603</Label>
        <Label for="passThrough">passthrough: false</Label>
        <Label for="Pos3">Pos3: { x: 0.0, y: 0.0, z: 0.0 }</Label>
        <Label for="pitch">pitch -> 0.0</Label>
        <Label for="yaw">yaw -> 0.0</Label>
      </div>
    </CardContent>
  </Card>
</template>

<script lang="ts" setup>
import { Card, CardContent } from '@/components/ui/card'
import { Label } from '@/components/ui/label'
</script>

<style lang="scss" scoped></style>
