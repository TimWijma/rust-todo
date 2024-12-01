<script lang="ts">
    import { Todo } from "./DTO/Todo";
    import { Fetch } from "./scripts/fetch";
    import { BACKEND_URL } from "./store";

    let todos: Todo[] = [];

    const getTodos = async () => {
        await Fetch.get(`${BACKEND_URL}/todos`).then((response: Todo[]) => {
            todos = response;
        });
    };

    const addTodo = async () => {
        await Fetch.post(`${BACKEND_URL}/todos`, {
            title: "New Todo",
            completed: false,
        }).then((response) => {
            getTodos();
        });
    };

    getTodos();
</script>

<main>
    <button on:click={addTodo}>Add todo</button>

    {#each todos as todo}
        <p>
            {todo.title}
        </p>
    {/each}
</main>
