#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// 1. Ubah struktur data menjadi Todo
#[contracttype]
#[derive(Clone, Debug)]
pub struct Todo {
    id: u64,
    title: String,
    is_completed: bool, // Menandakan apakah tugas sudah selesai
}

// Storage key untuk data todo list
const TODO_DATA: Symbol = symbol_short!("TODO_DATA");

#[contract]
pub struct TodoContract;

#[contractimpl]
impl TodoContract {
    
    // Mengambil semua daftar to-do
    pub fn get_todos(env: Env) -> Vec<Todo> {
        env.storage().instance().get(&TODO_DATA).unwrap_or(Vec::new(&env))
    }

    // Fungsi untuk membuat to-do baru
    pub fn create_todo(env: Env, title: String) -> String {
        let mut todos: Vec<Todo> = env.storage().instance().get(&TODO_DATA).unwrap_or(Vec::new(&env));
        
        let new_todo = Todo {
            id: env.prng().gen::<u64>(), // Generate ID unik otomatis
            title: title,
            is_completed: false, // Default: belum selesai
        };
        
        todos.push_back(new_todo);
        env.storage().instance().set(&TODO_DATA, &todos);
        
        String::from_str(&env, "To-do berhasil ditambahkan")
    }

    // Fungsi untuk mengubah status to-do (Selesai / Belum Selesai)
    pub fn toggle_todo_status(env: Env, id: u64) -> String {
        let mut todos: Vec<Todo> = env.storage().instance().get(&TODO_DATA).unwrap_or(Vec::new(&env));

        for i in 0..todos.len() {
            let mut todo = todos.get(i).unwrap();
            if todo.id == id {
                // Balikkan status boolean-nya
                todo.is_completed = !todo.is_completed;
                
                // Update item di dalam Vector Soroban
                todos.set(i, todo);
                
                // Simpan kembali ke storage
                env.storage().instance().set(&TODO_DATA, &todos);
                return String::from_str(&env, "Status to-do berhasil diperbarui");
            }
        }

        String::from_str(&env, "To-do tidak ditemukan")
    }

    // Fungsi untuk mengedit teks judul/konten to-do
    pub fn update_todo_title(env: Env, id: u64, new_title: String) -> String {
        let mut todos: Vec<Todo> = env.storage().instance().get(&TODO_DATA).unwrap_or(Vec::new(&env));

        for i in 0..todos.len() {
            let mut todo = todos.get(i).unwrap();
            if todo.id == id {
                todo.title = new_title;
                
                todos.set(i, todo);
                env.storage().instance().set(&TODO_DATA, &todos);
                return String::from_str(&env, "Judul to-do berhasil diubah");
            }
        }

        String::from_str(&env, "To-do tidak ditemukan")
    }

    // Fungsi untuk menghapus to-do berdasarkan id
    pub fn delete_todo(env: Env, id: u64) -> String {
        let mut todos: Vec<Todo> = env.storage().instance().get(&TODO_DATA).unwrap_or(Vec::new(&env));

        for i in 0..todos.len() {
            if todos.get(i).unwrap().id == id {
                todos.remove(i);

                env.storage().instance().set(&TODO_DATA, &todos);
                return String::from_str(&env, "Berhasil menghapus to-do");
            }
        }

        String::from_str(&env, "To-do tidak ditemukan")
    }
}

mod test;