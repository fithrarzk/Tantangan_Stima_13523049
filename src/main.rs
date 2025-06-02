use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct TSPResult {
    min_cost: i32,
    path: Vec<usize>,
}

struct TSPSolver {
    n: usize,
    dist: Vec<Vec<i32>>,
    memo: HashMap<(usize, usize), i32>, 
    parent: HashMap<(usize, usize), usize>, 
}

impl TSPSolver {
    fn new(distance_matrix: Vec<Vec<i32>>) -> Self {
        let n = distance_matrix.len();
        TSPSolver {
            n,
            dist: distance_matrix,
            memo: HashMap::new(),
            parent: HashMap::new(),
        }
    }

    fn dp(&mut self, mask: usize, pos: usize) -> i32 {
        if mask == (1 << self.n) - 1 {
            return self.dist[pos][0];
        }

        if let Some(&cost) = self.memo.get(&(mask, pos)) {
            return cost;
        }

        let mut min_cost = i32::MAX;
        let mut best_next = 0;

        for next in 0..self.n {
            if (mask & (1 << next)) == 0 {
                let new_mask = mask | (1 << next);
                let cost = self.dist[pos][next] + self.dp(new_mask, next);
                
                if cost < min_cost {
                    min_cost = cost;
                    best_next = next;
                }
            }
        }

        self.memo.insert((mask, pos), min_cost);
        self.parent.insert((mask, pos), best_next);
        
        min_cost
    }

    fn reconstruct_path(&self) -> Vec<usize> {
        let mut path = vec![0]; 
        let mut mask = 1;
        let mut pos = 0;

        while mask != (1 << self.n) - 1 {
            if let Some(&next) = self.parent.get(&(mask, pos)) {
                path.push(next);
                mask |= 1 << next;
                pos = next;
            } else {
                break;
            }
        }

        path.push(0);
        path
    }

    fn solve(&mut self) -> TSPResult {
        let min_cost = self.dp(1, 0);
        let path = self.reconstruct_path();
        
        TSPResult { min_cost, path }
    }
}

fn read_matrix() -> Vec<Vec<i32>> {
    println!("Masukkan jumlah kota:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Gagal membaca input");
    let n: usize = input.trim().parse().expect("Input harus berupa angka");

    println!("Masukkan matriks ketetanggaan ({} x {}):", n, n);
    println!("Format: masukkan {} angka untuk setiap baris, dipisahkan spasi", n);
    
    let mut matrix = Vec::new();
    
    for i in 0..n {
        println!("Baris {} (jarak dari kota {} ke kota lain):", i + 1, i + 1); 
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");
        
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Input harus berupa angka"))
            .collect();
            
        if row.len() != n {
            panic!("Jumlah elemen dalam baris harus {}", n);
        }
        
        matrix.push(row);
    }
    
    matrix
}

fn print_matrix(matrix: &Vec<Vec<i32>>) {
    println!("\nMatriks Ketetanggaan:");
    
    for (i, row) in matrix.iter().enumerate() {
        print!("{:2}: ", i + 1); 
        for &val in row {
            print!("{:4} ", val);
        }
        println!();
    }
}

fn print_result(result: &TSPResult) {
    println!("\n=== HASIL TSP ===");
    println!("Biaya minimum: {}", result.min_cost);
    
    let display_path: Vec<String> = result.path.iter()
        .map(|&x| (x + 1).to_string()) 
        .collect();
    
    println!("Jalur optimal: {}", display_path.join(" -> "));
    
    println!("\nDetail perjalanan:");
    for i in 0..result.path.len() - 1 {
        let from = result.path[i] + 1;  
        let to = result.path[i + 1] + 1;  
        println!("  Kota {} -> Kota {}", from, to);
    }
}

fn demo_with_predefined_matrices() {
    loop {
        println!("\n=== DEMO TSP DENGAN MATRIKS PREDEFINED ===");
        println!("1. 3 Kota (Simple Triangle)");
        println!("2. 4 Kota (Symmetric)");
        println!("3. 5 Kota (Asymmetric)");
        println!("4. 6 Kota (Complex)");
        println!("5. Kembali ke menu utama");
        print!("Pilih test case (1-6): ");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");
        
        match input.trim() {
            "1" => {
                println!("\n🔹 Test Case: 3 Kota (Simple)");
                let matrix = vec![
                    vec![0, 5, 8],
                    vec![5, 0, 3],
                    vec![8, 3, 0],
                ];
                run_test("3 Kota Triangle", matrix);
            }
            "2" => {
                println!("\n🔹 Test Case: 4 Kota");
                let matrix = vec![
                    vec![0, 10, 15, 20],
                    vec![10, 0, 35, 25],
                    vec![15, 35, 0, 30],
                    vec![20, 25, 30, 0],
                ];
                run_test("4 Kota (Symmetric)", matrix);
            }
            "3" => {
                println!("\n🔹 Test Case: 5 Kota");
                let matrix = vec![
                    vec![0, 2, 9, 10, 7],
                    vec![1, 0, 6, 4, 3],
                    vec![15, 7, 0, 8, 3],
                    vec![6, 3, 12, 0, 11],
                    vec![7, 8, 4, 2, 0],
                ];
                run_test("5 Kota (Asymmetric)", matrix);
            }
            "4" => {
                println!("\n🔹 Test Case: 6 Kota");
                let matrix = vec![
                    vec![0, 20, 42, 25, 30, 45],
                    vec![20, 0, 30, 34, 12, 8],
                    vec![42, 30, 0, 10, 28, 15],
                    vec![25, 34, 10, 0, 15, 40],
                    vec![30, 12, 28, 15, 0, 22],
                    vec![45, 8, 15, 40, 22, 0],
                ];
                run_test("6 Kota Complex", matrix);
            }
            "5" => {
                break;
            }
            _ => {
                println!("Pilihan tidak valid. Silakan pilih 1-6.");
            }
        }
    }
}

fn run_test(test_name: &str, matrix: Vec<Vec<i32>>) {
    println!("--- {} ---", test_name);
    print_matrix(&matrix);
    
    let start_time = std::time::Instant::now();
    let mut solver = TSPSolver::new(matrix);
    let result = solver.solve();
    let duration = start_time.elapsed();
    
    print_result(&result);
    
    println!("⏱️  Waktu eksekusi: {:.2?}", duration);
    println!("📊 States dievaluasi: {}", solver.memo.len());
    println!("{}", "=".repeat(50));
}

fn interactive_menu() {
    loop {
        println!("\n=== TSP SOLVER MENU ===");
        println!("1. Input matriks manual");
        println!("2. Jalankan demo dengan matriks predefined");
        println!("3. Keluar");
        print!("Pilih opsi (1-3): ");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");
        
        match input.trim() {
            "1" => {
                let matrix = read_matrix();
                print_matrix(&matrix);
                
                println!("\nMemproses...");
                let mut solver = TSPSolver::new(matrix);
                let result = solver.solve();
                print_result(&result);
                
                println!("\n=== INFORMASI ALGORITMA ===");
                println!("Algoritma: Dynamic Programming");
                println!("Kompleksitas waktu: O(n² × 2ⁿ)");
                println!("Kompleksitas ruang: O(n × 2ⁿ)");
                println!("Jumlah state yang dievaluasi: {}", solver.memo.len());
            }
            "2" => {
                demo_with_predefined_matrices();
            }
            "3" => {
                println!("Terima kasih!");
                break;
            }
            _ => {
                println!("Pilihan tidak valid. Silakan pilih 1-3.");
            }
        }
    }
}

fn main() {
    println!("=== TRAVELLING SALESMAN PROBLEM (TSP) ===");
    println!("Menggunakan Dynamic Programming");
    
    interactive_menu();
}