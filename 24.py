from z3 import *

def part_2(input):
	s = Solver()
	x, y, z = Reals("x y z")
	vx, vy, vz = Reals("vx vy vz")
	t0, t1, t2 = Reals("t0 t1 t2")
	


s = Solver()


x, y, z = Reals("x y z")
vx, vy, vz = Reals("vx vy vz")

t0, t1, t2 = Reals("t0 t1 t2")

# 19, 13, 30 @ -2,  1, -2
# 18, 19, 22 @ -1, -1, -2
# 20, 25, 34 @ -2, -2, -4

# s.add(19 + t0 * -2 == x + t0 * vx)
# s.add(13 + t0 * 1 == y + t0 * vy)
# s.add(30 + t0 * -2 == z + t0 * vz)

# s.add(18 + t1 * -1 == x + t1 * vx)
# s.add(19 + t1 * -1 == y + t1 * vy)
# s.add(22 + t1 * -2 == z + t1 * vz)

# s.add(20 + t2 * -2 == x + t2 * vx)
# s.add(25 + t2 * -2 == y + t2 * vy)
# s.add(34 + t2 * -4 == z + t2 * vz)


# 212542581053874, 357959731032403, 176793474286781 @ -88, -256, -240
# 154677220587564, 207254130208265, 139183938188421 @ 184, 74, 235
# 216869547613134, 38208083662943, 397740686492049 @ 109, 262, -66

s.add(212542581053874 + t0 * -88 == x + t0 * vx)
s.add(357959731032403 + t0 * -256 == y + t0 * vy)
s.add(176793474286781 + t0 * -240 == z + t0 * vz)

s.add(154677220587564 + t1 * 184 == x + t1 * vx)
s.add(207254130208265 + t1 * 74 == y + t1 * vy)
s.add(139183938188421 + t1 * 235 == z + t1 * vz)

s.add(216869547613134 + t2 * 109 == x + t2 * vx)
s.add(38208083662943 + t2 * 262 == y + t2 * vy)
s.add(397740686492049 + t2 * -66 == z + t2 * vz)

print(s.check())

m = s.model()
print(m)

print(m[x] + m[y] + m[z])

a_x = int(m[x].__str__())
a_y = int(m[y].__str__())
a_z = int(m[z].__str__())

print(a_z + a_x + a_y)