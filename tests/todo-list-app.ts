import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TodoListApp } from "../target/types/todo_list_app";
import {assert} from "chai";

describe("todo-list-app", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  // initializing the todolist app for test
  const program = anchor.workspace.TodoListApp as Program<TodoListApp>;
  // setting the auhtor
  const author = program.provider as anchor.AnchorProvider;

  it("can create a task", async () => {
    // Add your test here.
    const task = anchor.web3.Keypair.generate();
    const tx = await program.methods.addingTask("You have to take a shit").accounts({task: task.publicKey, systemProgram: anchor.web3.SystemProgram.programId,}).signers([task]).rpc();
    console.log("Your transaction signature", tx);

    const taskAccount = await program.account.task.fetch(task.publicKey);
    console.log("Your task", taskAccount);

    assert.equal(taskAccount.author.toBase58(),author.wallet.publicKey.toBase58());
    assert.equal(taskAccount.text, "You have to take a shit");
    assert.equal(taskAccount.isDone, false);
    assert.ok(taskAccount.createdAt);
    assert.ok(taskAccount.updatedAt);

  });
});
