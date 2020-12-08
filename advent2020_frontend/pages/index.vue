<template>
  <section class="index">
    <div class="index__card">
      <div class="index__card__body">
        <span class="index__card__body__text">USER ID: {{ userId }}</span>
        <span class="index__card__body__text">COUNT: {{ count }}</span>
      </div>
      <div class="index__card__button-container">
        <Button
          class="index__card__button-container__button__login"
          @click.native="login"
        >
          login
        </Button>
        <Button
          class="index__card__button-container__button__change"
          @click.native="change"
        >
          change
        </Button>
        <Button
          class="index__card__button-container__button__add"
          @click.native="add"
        >
          add
        </Button>
        <Button
          class="index__card__button-container__button__reset"
          @click.native="reset"
        >
          reset
        </Button>
      </div>
    </div>
  </section>
</template>

<script>
export default {
  components: {
    Button: () => import('@/components/base/Button.vue'),
  },
  data() {
    return {
      userId: '',
      count: 0,
    }
  },
  methods: {
    login() {
      this.$axios.post('/login').then(({ data }) => {
        this.userId = data.user_id
      })
    },
    change() {
      this.$axios.patch('/change').then(({ data }) => {
        this.userId = data.user_id
      })
    },
    add() {
      this.$axios.post('/add').then(({ data }) => {
        this.count = data.count
      })
    },
    reset() {
      this.$axios.delete('/reset').then(({ data }) => {
        this.count = data.count
      })
    },
  },
}
</script>

<style lang="scss" scoped>
.index {
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;

  &__card {
    width: 340px;
    padding: 12px 24px;
    background-color: #fffffe;
    border-radius: 5px;
    box-shadow: 0 2px 5px #ccc;
    display: flex;
    flex-direction: column;

    &__body {
      margin: 32px 0;
      display: flex;
      flex-direction: column;

      &__text {
        color: #272343;
        font-size: large;
      }
    }

    &__button-container {
      display: grid;
      grid-template-columns: 1fr 1fr;
      grid-template-rows: 1fr 1fr 1fr;
      grid-template-areas:
        'login .'
        'change .'
        'add reset';
      gap: 8px;

      &__button__login {
        grid-area: login;
      }

      &__button__change {
        grid-area: change;
      }

      &__button__add {
        grid-area: add;
      }

      &__button__reset {
        grid-area: reset;
      }
    }
  }
}
</style>
