<template>
  <Card>
    <CardContent class="space-y-5 mt-5">
      <div class="space-y-1">
        <Label for="current">Current password</Label>
        <Input id="current" type="password" />
      </div>
    </CardContent>
    <CardFooter>
      <Button>Save password</Button>
    </CardFooter>
  </Card>
</template>

<script lang="ts" setup>
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
</script>

<style lang="scss" scoped></style>
