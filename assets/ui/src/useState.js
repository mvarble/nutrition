import create from 'zustand';

const useState = create(set => ({
  mealStore: null,
  mealDiffs: null,
}))

export default useState;
