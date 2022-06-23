package com.devonfw.application.domain.repositories;

import org.springframework.data.domain.Page;
import org.springframework.data.jpa.repository.JpaRepository;

import com.devonfw.application.domain.models.AccessCodeEntity;
import com.devonfw.application.domain.tos.AccessCodeSearchCriteriaTo;

/**
 * {@link DefaultRepository} for {@link AccessCodeEntity}
 */
public interface AccessCodeRepository extends JpaRepository<AccessCodeEntity, Long> {

  /**
   * @param criteria the {@link AccessCodeSearchCriteriaTo} with the criteria to
   *                 search.
   * @return the {@link Page} of the {@link AccessCodeEntity} objects that matched
   *         the search. If no pageable is set, it
   *         will return a unique page with all the objects that matched the
   *         search.
   */
  public Page<AccessCodeEntity> findByCriteria(AccessCodeSearchCriteriaTo criteria);

  /**
   * Add sorting to the given query on the given alias
   *
   * @param query to add sorting to
   * @param alias to retrieve columns from for sorting
   * @param sort  specification of sorting
   */
  // public void addOrderBy(JPAQuery<AccessCodeEntity> query, AccessCodeEntity alias, Sort sort);

}