Приложение для поиска решения головоломки 15 (пятнашки) произвольного размера.
Особенности:

    стартовое значение головоломки генерируется случайным образом;

    есть проверка на возможность получения решения для стартового значения;

    для каждой вершины графа сохраняется не полный пройденный путь, а только хеш значение состояния (текущего, и предыдущего);

    реализован алгоритм поиска по наилудшему совпадению на графе A* с использованием многопоточной сортировки открытых вершин.

Сортировка реализована по функции F(x) = G(x) + H(x), G(x) - количество пройденных ходов, H(x) - манхэттенское расстояние.
Для ускорения поиска решения можно изменить сортировку по значению G(x) в файле puzzle.rs поменяв f на h.
let best = map.par_iter().min_by(|(k1, s1),(k2,s2)| (s1.set.f.cmp(&s2.set.f)));

Зависимости:
rand = "0.3.0"
rayon = "1.1"

После сборки запуск приложения puzzle <горизонтальный размер пазла> <вертикальный размер пазла>


An application for finding solutions to a puzzle of 15 (game tag) of arbitrary size.
Features:

    the starting value of the puzzle is randomly generated;

    there is a check for the possibility of obtaining a solution for the starting value;

    for each vertex of the graph, not the full path, but only the hash value of the state (current and previous) is saved;

    implemented the algorithm for finding the best match on the column A* using multi-thread sorting of open vertices.

The sorting is implemented by the function F (x) = G (x) + H (x), G (x) is the number of passes, H (x) is the Manhattan distance.
To speed up the search for a solution, you can change the sorting by the value of G (x) in the puzzle.rs file by changing f to h.
let best = map.par_iter (). min_by (| (k1, s1), (k2, s2) | (s1.set.f.cmp (& s2.set.f)));

Dependencies:
rand = "0.3.0"
rayon = "1.1"

After assembly, start up the puzzle application <horizontal puzzle size> <vertical puzzle size>

Пример: (Example:)

bash-3.2$ ./puzzle 3 3
My puzzle on size: 3, 3
[4, 3, 6]
[7, 8, 5]
[1, 2, 0]

-------RESULT-------

Step - 0
[4, 3, 6]
[7, 8, 5]
[1, 2, 0]

Step - 1
[4, 3, 6]
[7, 8, 0]
[1, 2, 5]

Step - 2
[4, 3, 6]
[7, 0, 8]
[1, 2, 5]

Step - 3
[4, 3, 6]
[7, 2, 8]
[1, 0, 5]

Step - 4
[4, 3, 6]
[7, 2, 8]
[0, 1, 5]

Step - 5
[4, 3, 6]
[0, 2, 8]
[7, 1, 5]

Step - 6
[4, 3, 6]
[2, 0, 8]
[7, 1, 5]

Step - 7
[4, 3, 6]
[2, 1, 8]
[7, 0, 5]

Step - 8
[4, 3, 6]
[2, 1, 8]
[7, 5, 0]

Step - 9
[4, 3, 6]
[2, 1, 0]
[7, 5, 8]

Step - 10
[4, 3, 0]
[2, 1, 6]
[7, 5, 8]

Step - 11
[4, 0, 3]
[2, 1, 6]
[7, 5, 8]

Step - 12
[4, 1, 3]
[2, 0, 6]
[7, 5, 8]

Step - 13
[4, 1, 3]
[0, 2, 6]
[7, 5, 8]

Step - 14
[0, 1, 3]
[4, 2, 6]
[7, 5, 8]

Step - 15
[1, 0, 3]
[4, 2, 6]
[7, 5, 8]

Step - 16
[1, 2, 3]
[4, 0, 6]
[7, 5, 8]

Step - 17
[1, 2, 3]
[4, 5, 6]
[7, 0, 8]

Step - 18
[1, 2, 3]
[4, 5, 6]
[7, 8, 0]