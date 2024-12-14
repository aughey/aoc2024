from manim import *
import random


class CreateCircle(MovingCameraScene):
    def construct(self):
        grid_size = (10,10)

        circle_size = 0.25
        grid_of_circles = VGroup(*[
            Square().scale(circle_size)
            for j in range(100)
        ]).arrange_in_grid(*grid_size, buff=0.1)

        self.play(Create(grid_of_circles, run_time=2))
        self.wait()
        #self.animate(0, 1, grid_of_circles, speed=1)
        #self.animate(1, 9, grid_of_circles, speed=0.1)

        filled_positions = []
        for y in range(10):
            for i in range(3):
                for x in range(3):
                    x = x + i*4
                    if x < 10 and random.getrandbits(1):
                        filled_positions.append([x,y])

        # Animate filling in the circles
        robots = VGroup(*[
            SVGMobject("robot").scale(0.20).move_to(grid_of_circles[x + y*10]).set_color(ORANGE)
            for x, y in filled_positions
        ])
        self.play(FadeIn(robots), lag_ratio=0.1, run_time=2)
        

        self.camera.frame.save_state()
        self.play(self.camera.frame.animate.set(width = 8).move_to(grid_of_circles[13]))
        self.animate(0,2, grid_of_circles, speed=1, robots=filled_positions)

        # Animate restore the camera
        self.play(self.camera.frame.animate.restore(), run_time=2)

        self.animate(2,8, grid_of_circles, speed=0.0334, robots=filled_positions)

        for _ in range(3):
            rt = 0.0334
            self.play(FadeOut(robots), run_time=rt)

            filled_positions = []
            for y in range(10):
                for i in range(3):
                    for x in range(3):
                        x = x + i*4
                        if x < 10 and random.getrandbits(1):
                            filled_positions.append([x,y])

            # Animate filling in the circles
            robots = VGroup(*[
                SVGMobject("robot").scale(0.20).move_to(grid_of_circles[x + y*10]).set_color(ORANGE)
                for x, y in filled_positions
            ])
            self.play(FadeIn(robots), lag_ratio=0.1, run_time=rt)
            self.animate(0,10, grid_of_circles, speed=rt, robots=filled_positions)

        filled_positions = []
        for y in range(10):
            for i in range(3):
                for x in range(3):
                    x = x + i*4
                    if x < 10 and random.getrandbits(1):
                        filled_positions.append([x,y])
        
        filled_positions.append([3,1])
        filled_positions.append([4,1])
        filled_positions.append([5,1])
        filled_positions.append([6,1])

        # Animate filling in the circles
        robots = VGroup(*[
            SVGMobject("robot").scale(0.20).move_to(grid_of_circles[x + y*10]).set_color(ORANGE)
            for x, y in filled_positions
        ])
        self.play(FadeIn(robots), lag_ratio=0.1, run_time=2)
        
        self.camera.frame.save_state()
        self.play(self.camera.frame.animate.set(width = 8).move_to(grid_of_circles[13]))
        self.animate(0,2, grid_of_circles, speed=1, robots=filled_positions)


    def animate(self, ystart, yrange, grid_of_circles, speed =1, robots=None):
        self.auto_zoom = False

        arrow = Arrow()
        first_grid_position = grid_of_circles[0].get_center()
        arrow.next_to(first_grid_position, LEFT)

        self.play(Create(arrow), run_time=speed)

        # Create a box around the first 4 circles
        box = SurroundingRectangle(VGroup(*grid_of_circles[:4]))
        self.play(Create(box), run_time=speed)  

        for y in range(yrange):
            y = ystart + y
            grid_position = grid_of_circles[y*10].get_center()
            # Animate arrow to this new position
            self.play(arrow.animate.next_to(grid_position, LEFT), run_time=speed)
            for x in range(10-4+1):
                circles = grid_of_circles[y*10 + x:y*10 + x + 4]
                # Move box to surround these circles
                self.play(box.animate.surround(circles).set_color(YELLOW).set_fill(BLACK, opacity=0), run_time=speed)
                
                question_marks = VGroup()
                found = 0
                for (i,circle) in enumerate(circles):
                    # Put a question mark in this circle
                    question_mark = MathTex("?").scale(0.5)
                    question_mark.move_to(circle)
                    # Fade in this question mark and add it to the group
                    self.play(FadeIn(question_mark), run_time=speed)
                    question_marks.add(question_mark)

                    if robots is not None:
                        xy = [x+i, y]
                        if xy in robots:
                            # Change question mark into a check mark
                            check_mark = MathTex("\\checkmark")
                            check_mark.move_to(circle).set_color(GREEN)
                            self.play(Transform(question_mark, check_mark), run_time=speed)
                            question_marks.add(check_mark)
                            found += 1
                        else:
                            # Change question mark into a cross
                            cross = MathTex("\\times")
                            cross.move_to(circle).set_color(RED)
                            self.play(Transform(question_mark, cross), run_time=speed)
                            question_marks.add(cross)
                            break
                
                if found == 4:
                    self.play(box.animate.set_color(GREEN).set_fill(GREEN,opacity=0.5), run_time=speed)
                    break
                else:
                    self.play(box.animate.set_color(RED).set_fill(RED,opacity=0.5), run_time=speed)

                # Fade out and remove all question marks
                self.play(FadeOut(question_marks), run_time=speed)
        self.play(FadeOut(arrow), FadeOut(box), run_time=speed)


