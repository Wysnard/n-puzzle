def spiral_traversal(matrix):
	res = []
	if len(matrix) == 0:
		return res
	row_begin = 0
	row_end = len(matrix) - 1
	col_begin = 0
	col_end = len(matrix[0]) - 1

	while row_begin <= row_end and col_begin <= col_end:
		print("#1")
		for i in range(col_begin, col_end+1):
			print(col_begin, i)
			res.append(matrix[row_begin][i])
		row_begin += 1

		print('#2')
		for i in range(row_begin, row_end+1):
			print(i, col_end)
			res.append(matrix[i][col_end])
		col_end -= 1

		print('#3')
		if row_begin <= row_end:
			for i in range(col_end, col_begin-1, -1):
				print(row_end, i)
				res.append(matrix[row_end][i])
		row_end -= 1

		print('#4')
		if col_begin <= col_end:
			for i in range(row_end, row_begin-1, -1):
				print(i, col_begin)
				res.append(matrix[i][col_begin])
		col_begin += 1

	return res

mat = [[0, 1, 2], [3, 4, 5], [6, 7, 8]]

# [1, 2, 3, 6, 9, 8, 7, 4, 5]
print(spiral_traversal(mat))