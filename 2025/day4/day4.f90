program day4
    implicit none
    character(len=256), dimension(:), allocatable :: lines
    integer :: io

    open(newunit=io, file="day4.txt", status="old", action="read")

    block
        integer :: ios, num_lines
        num_lines = 0
        do
            read(io, *, iostat=ios)
            if (ios/= 0) exit
            num_lines = num_lines + 1
        end do
        rewind(io)

        allocate(lines(num_lines))
    end block

    block
        integer :: i
        do i = 1, size(lines)
            read(io, *) lines(i)
        end do
    end block

    block
        integer :: part1_result
        real :: start_time, end_time
        call cpu_time(start_time)
        part1_result = part1(lines)
        call cpu_time(end_time)
        print *, part1_result, " Time taken: ", end_time - start_time
    end block

    block
        integer*8 :: part2_result
        real :: start_time, end_time
        call cpu_time(start_time)
        part2_result = part2(lines)
        call cpu_time(end_time)
        print *, part2_result, " Time taken: ", end_time - start_time
    end block

contains

    function part1(grid) result(count)
        implicit none
        character(len=256), dimension(:), intent(in) :: grid
        integer :: count, x, y

        count = 0

        ! Check all grid positions
        do y = 1, size(grid)
            do x = 1, len_trim(grid(y))
                ! If theres a roll, and its adjacent to less than 4 other rolls, count it
                if (grid(y)(x:x) == '@' .and. count_adjacent(grid, x, y) < 4) then
                    count = count + 1
                end if
            end do
        end do
    end function part1

    function part2(grid) result(count)
        implicit none
        character(len=256), dimension(:), intent(inout) :: grid
        integer :: count, x, y
        logical :: changed

        count = 0
        changed = .true.

        ! Repeat until we hit a fixed point
        ! At which point no more roll can be removed.
        ! This does have bounded runtime as each iteration will remove at least one roll until no more can be removed.
        do while (changed)
            changed = .false.
            ! We then do the same as part 1, but we remove rolls as we find them.
            do y = 1, size(grid)
                do x = 1, len_trim(grid(y))
                    if (grid(y)(x:x) == '@' .and. count_adjacent(grid, x, y) < 4) then
                        count = count + 1
                        ! Remove the roll and mark that we need to iterate again.
                        grid(y)(x:x) = '.'
                        changed = .true.
                    end if
                end do
            end do
        end do
    end function part2

    integer function count_adjacent(grid, x, y)
        implicit none 
        character(len=256), dimension(:), intent(in) :: grid
        integer, intent(in) :: x, y
        integer :: adjacent, dx, dy
        count_adjacent = 0

        do dy = -1, 1
            do dx = -1, 1
                if (dy == 0 .and. dx == 0) cycle
                if (x + dx < 1 .or. x + dx > len_trim(grid(y))) cycle
                if (y + dy < 1 .or. y + dy > size(grid)) cycle
                if (grid(y+dy)(x+dx:x+dx) == '@') then
                    count_adjacent = count_adjacent + 1
                end if
            end do
        end do
    end function count_adjacent
 
end program day4
