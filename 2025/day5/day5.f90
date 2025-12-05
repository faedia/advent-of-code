program day5
    implicit none
    type :: range_t
        integer*8 :: start
        integer*8 :: end
    end type range_t

    integer :: io
    type(range_t), dimension(:), allocatable :: ranges
    integer*8, dimension(:), allocatable :: ids

    open(newunit=io, file="day5.txt", status="old", action="read")

    block
        integer :: ios, num_ranges, num_ids
        character(len=100) :: line
        num_ranges = 0
        num_ids = 0
        do
            read(io, '(A)', iostat=ios) line
            if (ios /= 0) exit
            if (trim(line) == "") exit
            num_ranges = num_ranges + 1
        end do

        do
            read(io, *, iostat=ios) line
            if (ios /= 0) exit
            num_ids = num_ids + 1
        end do

        rewind(io)
        allocate(ranges(num_ranges))
        allocate(ids(num_ids))
    end block

    block
        integer :: i, dash_pos
        character(len=100) :: line
        do i = 1, size(ranges)
            read(io, '(A)') line
            dash_pos = index(line, '-')
            read(line(1:dash_pos-1), '(I32)') ranges(i)%start
            read(line(dash_pos+1:), '(I32)') ranges(i)%end
        end do

        do i = 1, size(ids)
            read(io, *) ids(i)
        end do
    end block

    block
        integer :: part1_result
        real :: start_time, end_time
        call cpu_time(start_time)
        part1_result = part1(ranges, ids)
        call cpu_time(end_time)
        print *, part1_result, " Time taken: ", end_time - start_time
    end block

    block
        integer*8 :: part2_result
        real :: start_time, end_time
        call cpu_time(start_time)
        part2_result = part2(ranges)
        call cpu_time(end_time)
        print *, part2_result, " Time taken: ", end_time - start_time
    end block

contains
    
    integer function part1(ranges, ids)
        implicit none
        type(range_t), dimension(:), intent(in) :: ranges
        integer*8, dimension(:), intent(in) :: ids
        integer*8 :: count, i, j

        count = 0

        ! Just go through each id and see if it falls within a range.
        ! If this was sorted then we could do a binary search to find if it falls within a range faster
        ! But this was an easier implementation.
        do i = 1, size(ids)
            do j = 1, size(ranges)
                ! If its within the range, increment count and continue
                if (ids(i) >= ranges(j)%start .and. ids(i) <= ranges(j)%end) then
                    count = count + 1
                    exit
                end if
            end do
        end do

        part1 = count
    end function part1

    integer*8 function part2(ranges)
        implicit none
        type(range_t), dimension(:), intent(inout) :: ranges
        integer*8 :: count
        integer*8 :: i, j
        integer*8 :: merged_count
        type(range_t) :: temp
        logical :: merged
        count = 0;
        merged_count = size(ranges)
        merged = .true.


        ! Keep merging overlapping ranges until no more merges occur
        ! This is not particularly efficient, but the input is small and the implementation is simple.
        ! Similar to part 1, if the ranges were sorted then we could do this more efficiently.
        do while (merged)
            merged = .false.
            do i = 1, size(ranges)
                do j = i + 1, merged_count
                    ! If the ids overlap, extend range i to include j and remove j from the list
                    if (ranges(i)%end >= ranges(j)%start .and. ranges(i)%start <= ranges(j)%end) then
                        ! They overlap, merge and remove j
                        ranges(i)%start = min(ranges(i)%start, ranges(j)%start)
                        ranges(i)%end = max(ranges(i)%end, ranges(j)%end)
                        ranges(j) = ranges(merged_count)
                        merged_count = merged_count - 1
                        merged = .true.
                    end if
                end do
            end do
        end do

        do i = 1, merged_count
            count = count + (ranges(i)%end - ranges(i)%start + 1)
        end do

        part2 = count

    end function part2

end program day5
