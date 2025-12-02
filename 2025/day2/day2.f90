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
                integer*8 :: num
                character(len=100) :: num_str
                ! For each number in the range, extract the number to a string
                ! Then we check to see if the The prefix of the string is equal to the suffix of the string!
                do num = numbers(i)%lhs, numbers(i)%rhs
                    write(num_str, '(I0.0)') num
                    if (trim(num_str(:len_trim(num_str)/2)) == trim(num_str((len_trim(num_str)/2)+1:))) then
                        count = count + num
                    end if
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
                integer*8 :: num
                character(len=100) :: num_str
                do num = numbers(i)%lhs, numbers(i)%rhs
                    ! Get that number into a string!
                    write(num_str, '(I0.0)') num

                    block
                        integer*8 :: j, length
                        logical :: is_invalid

                        ! For each possible substring size.
                        do length = 1, len_trim(num_str)/2
                            ! If we cannot fit a whole number of windows into the string then skip it.
                            if (mod(len_trim(num_str), length) /= 0) continue
                            
                            ! Starting assumption, think it is valid, this is turned false when we find a contradition.
                            is_invalid = .true.
                            ! For all distinct windows check that the strings are equal, this proves that the string is made of a repeating patter
                            do j = length + 1, len_trim(num_str), length
                                ! If its not equal we've found a counter example for this windows size fot his number so lets just continue.
                                if (num_str(1:length) /= num_str(j:j+length-1)) then
                                    is_invalid = .false.
                                    exit
                                end if
                            end do
                            if (is_invalid) exit
                        end do

                        ! If we went though all of that and we are still invalid so make sure to count the id!
                        if (is_invalid) then
                            count = count + num
                        end if
                    end block
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