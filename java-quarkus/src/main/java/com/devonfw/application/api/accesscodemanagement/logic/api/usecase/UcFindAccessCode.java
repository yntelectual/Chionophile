package com.devonfw.application.accesscodemanagement.logic.api.usecase;

import java.util.List;

import com.devonfw.application.accesscodemanagement.logic.api.to.AccessCodeEto;
import org.springframework.data.domain.Page;

import com.devonfw.application.accesscodemanagement.logic.api.to.AccessCodeCto;
import com.devonfw.application.accesscodemanagement.logic.api.to.AccessCodeSearchCriteriaTo;

public interface UcFindAccessCode {

  /**
   * Returns a composite AccessCode by its id 'id'.
   *
   * @param id The id 'id' of the AccessCode.
   * @return The {@link AccessCodeCto} with id 'id'
   */
  AccessCodeCto findAccessCodeCto(long id);

  /**
   * Returns a paginated list of composite AccessCodes matching the search
   * criteria.
   *
   * @param criteria the {@link AccessCodeSearchCriteriaTo}.
   * @return the {@link List} of matching {@link AccessCodeCto}s.
   */
  Page<AccessCodeCto> findAccessCodeCtos(AccessCodeSearchCriteriaTo criteria);

  /**
   * Returns a paginated list of AccessCodeEto matching the search criteria.
   *
   * @param criteria the {@link AccessCodeSearchCriteriaTo}.
   * @return the {@link List} of matching {@link AccessCodeEto}s.
   */
  Page<AccessCodeEto> findAccessCodeEtos(AccessCodeSearchCriteriaTo criteria);

}
