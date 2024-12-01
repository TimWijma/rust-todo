<script lang="ts">
    import { Todo } from "./DTO/Todo";
    import { Fetch } from "./scripts/fetch";
    import { BACKEND_URL } from "./store";

    let todos: Todo[] = [];

    let newTodo = new Todo();

    const getTodos = async () => {
        await Fetch.get(`${BACKEND_URL}/todos`).then((response: Todo[]) => {
            todos = response;
        });
    };

    const addTodo = async () => {
        await Fetch.post(`${BACKEND_URL}/todos`, newTodo).then(() => {
            getTodos();
        });
    };

    const updateTodo = async (todo: Todo) => {
        await Fetch.put(`${BACKEND_URL}/todos/${todo.id}`, todo).then(() => {
            getTodos();
        });
    };

    const deleteTodo = async (todo: Todo) => {
        await Fetch.delete(`${BACKEND_URL}/todos/${todo.id}`).then(() => {
            getTodos();
        });
    };

    getTodos();
</script>

<main>
    <input type="text" bind:value={newTodo.title} />

    <button on:click={addTodo}>Add todo</button>

    {#each todos as todo}
        <div>
            {#if !todo.editing}
                <input type="checkbox" bind:checked={todo.completed} on:change={() => {
                    updateTodo(todo);
                }} />
                <span>
                    {todo.title}
                </span>
                <button on:click={() => (todo.editing = true)}>Edit</button>
                <button on:click={() => deleteTodo(todo)}>Delete</button>
            {:else}
                <input type="text" bind:value={todo.title} />
                <button on:click={() => updateTodo(todo)}>Save</button>
            {/if}
        </div>
    {/each}
</main>
