module day2_mod
    implicit none
    public part1, part2

    type :: nums
        integer*8 :: lhs
        integer*8 :: rhs
    end type nums

contains

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
                    half_idx = (int8(log10(real(num))) + 1) / 2
                    ! If the top half of the number equals the bottom half of the number then we are A Okay!
                    ! This constructos formulas of the form 1234 / 100 and 1234 % 100
                    ! Where the former would 1234 / 100 = 12 and the latter 1234 % 100 = 34.
                    if (num / (10**half_idx) == mod(num, 10**half_idx)) count = count + num
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
                    total_digits = (int8(log10(real(num))) + 1)
                    ! For each possible digit windows length from 1 to half the total number of digits
                    do digits = 1, total_digits / 2
                        block
                            integer*8 :: multiple,times,amount
                            ! Calculate the top digits amount value in num
                            multiple = num / (10**(total_digits - digits))
                            amount = 0
                            ! The go ahead and construct multiple pattern repeated for how many times we need to repeat digits
                            do times = 1, total_digits / digits
                                ! Pad to make sure there is enough space for multiple
                                amount = (amount * (10**digits)) + multiple
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
        result_part1 = part1(numbers)
        print *, result_part1
    end block

    block
        integer*8 :: result_part2
        result_part2 = part2(numbers)
        print *, result_part2
    end block

end program day2