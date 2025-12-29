module day2_mod
    implicit none
    public part1, part2

    type :: nums
        integer*8 :: lhs
        integer*8 :: rhs
    end type nums

contains

    integer function count_digits(num) result(count)
        implicit none
        integer*8, intent(in) :: num

        if (num < 10) then
            count = 1
        else if (num < 100) then
            count = 2
        else if (num < 1000) then
            count = 3
        else if (num < 10000) then
            count = 4
        else if (num < 100000) then
            count = 5
        else if (num < 1000000) then
            count = 6
        else if (num < 10000000) then
            count = 7
        else if (num < 100000000) then
            count = 8
        else if (num < 1000000000) then
            count = 9
        else
            count = 10
        end if


    end function count_digits

    integer*8 function pow10(exp) result(value)
        implicit none
        integer*8, intent(in) :: exp
        integer*8, dimension(0:9) :: powers = [1,10,100,1000,10000,100000,1000000,10000000,100000000,1000000000]

        value = powers(exp)
    end function pow10

    function part1(numbers) result(count)
        implicit none
        type(nums), dimension(:), intent(in) :: numbers
        integer*8 :: count, i

        count = 0

        ! For all set of numbers!!
        do i = 1, size(numbers)
            block
                integer*8 :: num, half_idx
                do num = numbers(i)%lhs, numbers(i)%rhs
                    half_idx = (count_digits(num)) / 2
                    ! If the top half of the number equals the bottom half of the number then we are A Okay!
                    ! This constructos formulas of the form 1234 / 100 and 1234 % 100
                    ! Where the former would 1234 / 100 = 12 and the latter 1234 % 100 = 34.
                    if (num / pow10(half_idx) == mod(num, pow10(half_idx))) count = count + num
                end do
            end block
        end do
    end function part1

    function part2(numbers) result(count)
        implicit none
        type(nums), dimension(:), intent(in) :: numbers
        integer*8 :: count, i

        count = 0

        do i = 1, size(numbers)
            block
                integer*8 :: num, total_digits, digits
                do num = numbers(i)%lhs, numbers(i)%rhs
                    total_digits = (count_digits(num))
                    ! For each possible digit windows length from 1 to half the total number of digits
                    do digits = 1, total_digits / 2
                        block
                            integer*8 :: multiple,times,amount
                            ! Calculate the top digits amount value in num
                            multiple = num / (pow10(total_digits - digits))
                            amount = 0
                            ! The go ahead and construct multiple pattern repeated for how many times we need to repeat digits
                            do times = 1, total_digits / digits
                                ! Pad to make sure there is enough space for multiple
                                amount = (amount * pow10(digits)) + multiple
                            end do
                            ! If the total amount is the same as num then woooooooo our num is a repeating pattern!!
                            if (num == amount) then
                                count = count + num
                                exit
                            end if
                        end block
                    end do
                end do
            end block
        end do

    end function part2

end module day2_mod

program day2
    use day2_mod
    implicit none
    integer*8 :: io, ios
    character(len=100000) :: line
    type(nums), dimension(:), allocatable :: numbers
    integer*8 :: pos
    integer*8 :: items, i
    items = 0
    pos = 1

    open(newunit=io, file="day2.txt", status="old", action="read")

    ! God I hate string handling in fortran!
    do
        read(io, '(A)', iostat=ios) line
        if (ios /= 0) exit
    end do

    do while (pos <= len_trim(line))
        block
            integer*8 :: comma_pos
            comma_pos = index(line(pos:), ',') + pos - 1
            items = items + 1
            if (comma_pos == pos - 1) exit
            pos = comma_pos + 1
        end block
    end do

    pos = 1
    allocate(numbers(items))

    do i = 1, size(numbers)
        block
            integer*8 :: comma_pos, dash_pos
            comma_pos = index(line(pos:), ',') + pos - 1
            if (comma_pos == pos - 1) then
                comma_pos = len_trim(line) + 1
            end if
            dash_pos = index(line(pos:), '-') + pos - 1
            read(line(pos:dash_pos-1), *) numbers(i)%lhs
            read(line(dash_pos+1:comma_pos-1), *) numbers(i)%rhs

            pos = comma_pos + 1
        end block
    end do

    block
        integer*8 :: result_part1
        real :: start_time, end_time
        call cpu_time(start_time)
        result_part1 = part1(numbers)
        call cpu_time(end_time)
        print *, result_part1, " Time taken: ", end_time - start_time
    end block

    block
        integer*8 :: result_part2
        real :: start_time, end_time
        call cpu_time(start_time)
        result_part2 = part2(numbers)
        call cpu_time(end_time)
        print *, result_part2, " Time taken: ", end_time - start_time
    end block

end program day2
