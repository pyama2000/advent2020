<template>
  <button :class="buttonClass">
    <span class="button__text">
      <slot></slot>
    </span>
  </button>
</template>

<script>
export default {
  props: {
    size: {
      type: String,
      default: 'normal',
      validator: (value) => {
        return ['small', 'normal', 'large'].includes(value)
      },
    },
    color: {
      type: String,
      default: 'primary',
      validator: (value) => {
        return ['primary', 'secondary'].includes(value)
      },
    },
  },
  computed: {
    buttonClass() {
      const buttonSizeClass =
        this.size === 'normal' ? '' : `button--${this.size}`
      const buttonColorClass =
        this.color === 'primary' ? '' : `button--${this.color}`

      return ['button', buttonSizeClass, buttonColorClass]
    },
  },
}
</script>

<style lang="scss" scoped>
.button {
  width: 100px;
  height: 40px;
  background-color: #ffd803;
  color: #272343;
  font-weight: bold;
  text-transform: uppercase;
  transition: all 0.2s ease;
  border-style: none;
  border-radius: 5px;

  &--small {
    @extend .button;
    width: 80px;
    height: 30px;
    font-size: 8px;
  }

  &--large {
    @extend .button;
    width: 160px;
    height: 60px;
    font-size: 18px;
  }

  &--secondary {
    @extend .button;
    background-color: #e3f6f5;
    color: #272343;
  }

  &__text {
    opacity: 0.8;
  }
}
</style>
