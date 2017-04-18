import polygons

context = polygons.new_context()

points = [(0.0, 0.0),
          (1.0, 0.0),
          (2.0, 0.0),
          (3.0, 0.0),
          (4.0, 0.0),
          (5.0, 0.0),
          (6.0, 0.0),
          (7.0, 0.0),
          (8.0, 0.0),
          (8.1, 0.0),
          (8.2, 0.0),
          (8.3, 0.0),
          (8.4, 0.0),
          (8.5, 0.0),
          (8.6, 0.0),
          (8.7, 0.0),
          (8.8, 0.0),
          (8.9, 0.0),
          (9.0, 0.0),
          (9.1, 0.0),
          (9.2, 0.0),
          (9.3, 0.0),
          (9.4, 0.0),
          (9.5, 0.0),
          (4.0, 2.0),
          (0.0, 0.0)]

polygons.add_polygon(context, points)

d = polygons.get_distance(context, 10.0, 20.0)
print(d)

polygons.free_context(context)
