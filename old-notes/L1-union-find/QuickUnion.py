class QuickUnionUF:
  def __init__(self, N):
    self.N, self.id, self.size = N, [i for i in range(N)], [1]*N
    
  def union(self, p, q):
    p_root, q_root = self.root(p), self.root(q)
    self.id[p_root] = q_root
    
  def connected(self, p, q):
    return self.root(p) == self.root(q)

  def root(self, i):
    while self.id[i] != i: i = self.id[i]
    return i


class QuickUnionTests:
  def __init__(self):
    self.N = 2 #number of cases
    self.case = [None for i in range(self.N)]
    
    self.case[0] = {
      'connections': [(4, 3), (3, 8), (6, 5), (9, 4), (2, 1), (8, 9),
        (5, 0), (7, 2), (6, 1), (7, 3)],
      'id': [1, 8, 1, 8, 3, 0, 5, 1, 8, 8]
    }
    
    self.case[1] = {
      'connections': [(1, 2), (7, 9), (0, 4), (8, 0), (4, 6), (1, 9), (3, 4), 
      (7, 0), (0, 5)],
      'id': [4, 2, 9, 6, 6, 5, 5, 9, 4, 6]
    } 
  
  def split_case(self, k):
    connections = self.case[k]['connections']
    correct_id = self.case[k]['id']
    N = len(correct_id)

    return N, connections, correct_id

  def run_test(self, k):
    N, connections, correct_id = self.split_case(k)
    UF = QuickUnionUF(N)
        
    for x, y in connections:
      UF.union(x, y)
 
    return UF.id == correct_id
          
  def run_all_tests(self):
    for k in range(self.N):
      if self.run_test(k) is False:
        print("TEST FAILED: CASE", k)
      else:
        print("TEST PASSED: CASE", k)